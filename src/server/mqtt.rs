mod command_handler;
pub mod topic_suffix;
use anyhow::anyhow;
pub use command_handler::CommandHandler;
use dioxus::html::button::form;
use tracing::error;
use tracing::warn;

use crate::constants;
use crate::drivers::PowerSupplyDriver;
use crate::payload::payloads_generated::StatusCode;
use crate::payload::StatusBuilder;
use bytes::Bytes;
use pza_toolkit::rumqtt::client::init_client;
use pza_toolkit::rumqtt::client::RumqttCustomAsyncClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::trace;

#[derive(Debug)]
pub struct MqttRunnerHandler {
    pub task_handler: Arc<Option<tokio::task::JoinHandle<()>>>,
}

/// MQTT MqttRunner for handling power supply commands and measurements
pub struct MqttRunner {
    /// Current status of the instance
    status: StatusCode,

    /// MQTT client
    mqtt_client: RumqttCustomAsyncClient,
    /// MqttRunner name
    name: String,

    /// Driver MqttRunner
    driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,

    /// Base topic for the MQTT runner
    topic_base: String,

    /// psu/{name}/status
    topic_status: String,
    /// psu/{name}/error
    topic_error: String,

    /// psu/{name}/control/oe
    topic_control_oe: String,
    /// psu/{name}/control/oe/cmd"
    topic_control_oe_cmd: String,

    /// psu/{name}/control/voltage
    topic_control_voltage: String,
    /// psu/{name}/control/voltage/cmd
    topic_control_voltage_cmd: String,

    /// psu/{name}/control/voltage
    topic_control_current: String,
    /// psu/{name}/control/current/cmd
    topic_control_current_cmd: String,

    /// psu/{name}/measure/voltage/refresh_freq
    topic_measure_voltage_refresh_freq: String,
    /// psu/{name}/measure/current/refresh_freq
    topic_measure_current_refresh_freq: String,
}

impl MqttRunner {
    // --------------------------------------------------------------------------------

    /// Start the runner
    pub fn start(
        name: String,
        driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) -> anyhow::Result<MqttRunnerHandler> {
        let (client, event_loop) = init_client(format!("{}/{}", constants::SERVER_TYPE_NAME, name));

        let custom_client = RumqttCustomAsyncClient::new(
            client,
            rumqttc::QoS::AtMostOnce,
            true,
            format!("{}/{}", constants::SERVER_TYPE_NAME, name),
        );

        // Create runner object
        let runner = MqttRunner {
            status: StatusCode::Booting,
            name: name.clone(),
            driver,
            topic_base: custom_client.topic_with_prefix(""),
            topic_status: custom_client.topic_with_prefix("status"),
            topic_error: custom_client.topic_with_prefix(topic_suffix::ERROR),
            topic_control_oe: custom_client.topic_with_prefix("control/oe"),
            topic_control_oe_cmd: custom_client.topic_with_prefix("control/oe/cmd"),
            topic_control_voltage: custom_client.topic_with_prefix("control/voltage"),
            topic_control_voltage_cmd: custom_client.topic_with_prefix("control/voltage/cmd"),
            topic_control_current: custom_client.topic_with_prefix("control/current"),
            topic_control_current_cmd: custom_client.topic_with_prefix("control/current/cmd"),
            topic_measure_voltage_refresh_freq: custom_client
                .topic_with_prefix("measure/voltage/refresh_freq"),
            topic_measure_current_refresh_freq: custom_client
                .topic_with_prefix("measure/current/refresh_freq"),
            mqtt_client: custom_client,
        };

        let task_handler = tokio::spawn(Self::task_loop(event_loop, runner));

        Ok(MqttRunnerHandler {
            task_handler: Arc::new(Some(task_handler)),
        })
    }

    // --------------------------------------------------------------------------------

    /// The main async task loop for the MQTT runner
    async fn task_loop(mut event_loop: rumqttc::EventLoop, mut runner: MqttRunner) {
        // Publish status (booting)
        runner
            .publish_booting_status()
            .await
            .expect("Unable to publish status");

        // Subscribe to all relevant topics
        runner
            .mqtt_client
            .subscribe_to_all(vec![
                runner.topic_control_oe_cmd.clone(),
                runner.topic_control_voltage_cmd.clone(),
                runner.topic_control_current_cmd.clone(),
                runner.topic_measure_voltage_refresh_freq.clone(),
                runner.topic_measure_current_refresh_freq.clone(),
            ])
            .await
            .expect("Unable to subscribe command topics");

        // Main event loop
        loop {
            match runner.status {
                // ------------------------------------------------------------
                StatusCode::Booting => {
                    // In Booting status, continue processing messages
                    trace!("[{}] In Booting status, processing messages", runner.name);
                    match runner.initialize().await {
                        Ok(_) => {
                            trace!("[{}] Initialization successful", runner.name);
                            runner
                                .publish_running_status()
                                .await
                                .expect("Unable to set status to Running");
                        }
                        Err(e) => {
                            error!("[{}] Initialization failed: {}", runner.name, e);
                            runner
                                .publish_panicking_status(format!("Initialization failed: {}", e))
                                .await
                                .expect("Unable to set status to Panicking");
                        }
                    }
                }
                // ------------------------------------------------------------
                StatusCode::Running => {
                    // In Running status, continue processing messages
                    trace!("[{}] In Running status, processing messages", runner.name);
                    while let Ok(event) = event_loop.poll().await {
                        match event {
                            rumqttc::Event::Incoming(incoming) => match incoming {
                                rumqttc::Packet::Publish(packet) => {
                                    let topic = packet.topic;
                                    let payload = packet.payload;
                                    trace!(
                                        "[{}] Received message on topic: {}",
                                        runner.name,
                                        topic
                                    );
                                    match runner.handle_incoming_message(&topic, payload).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error!(
                                                "[{}] Error handling message on topic {}: {}",
                                                runner.name, topic, e
                                            );
                                            runner
                                                .publish_panicking_status(format!(
                                                    "Error handling message on topic {}: {}",
                                                    topic, e
                                                ))
                                                .await
                                                .expect("Unable to set status to Panicking");
                                        }
                                    }
                                }
                                _ => {}
                            },
                            rumqttc::Event::Outgoing(_outgoing) => {}
                        }
                    }
                }
                // ------------------------------------------------------------
                StatusCode::Panicking => {
                    // In Panicking status, stop processing messages
                    trace!(
                        "[{}] In Panicking status, stopping message processing",
                        runner.name
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
                _ => {
                    // For other statuses, just log and continue
                    trace!(
                        "[{}] In {:?} status, processing messages",
                        runner.name,
                        runner.status
                    );
                }
            }
        }
    }

    // --------------------------------------------------------------------------------

    /// Initialize the runner (if needed)
    async fn initialize(&self) -> anyhow::Result<()> {
        let mut driver = self.driver.lock().await;

        driver.initialize().await?;

        // Publish initial output enable state
        let oe_value = driver.output_enabled().await?;
        self.mqtt_client
            .client
            .publish(
                self.topic_control_oe.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(if oe_value { "ON" } else { "OFF" }),
            )
            .await?;

        // Get and check initial voltage setting
        let mut voltage = driver.get_voltage().await?;
        if let Ok(voltage_value) = voltage.parse::<f32>() {
            let mut adjusted_voltage = voltage_value;

            // Check against minimum voltage limit
            if let Some(min_voltage) = driver.security_min_voltage() {
                if voltage_value < min_voltage {
                    adjusted_voltage = min_voltage;
                }
            }

            // Check against maximum voltage limit
            if let Some(max_voltage) = driver.security_max_voltage() {
                if voltage_value > max_voltage {
                    adjusted_voltage = max_voltage;
                }
            }

            // If voltage was adjusted, set it in the driver
            if adjusted_voltage != voltage_value {
                voltage = adjusted_voltage.to_string();
                let _ = driver.set_voltage(voltage.clone()).await;
            }
        }

        self.mqtt_client
            .client
            .publish(
                self.topic_control_voltage.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(voltage),
            )
            .await?;

        // Get and check initial current setting
        let mut current = driver.get_current().await?;
        if let Ok(current_value) = current.parse::<f32>() {
            let mut adjusted_current = current_value;

            // Check against minimum current limit
            if let Some(min_current) = driver.security_min_current() {
                if current_value < min_current {
                    adjusted_current = min_current;
                }
            }

            // Check against maximum current limit
            if let Some(max_current) = driver.security_max_current() {
                if current_value > max_current {
                    adjusted_current = max_current;
                }
            }

            // If current was adjusted, set it in the driver
            if adjusted_current != current_value {
                current = adjusted_current.to_string();
                let _ = driver.set_current(current.clone()).await;
            }
        }

        self.mqtt_client
            .client
            .publish(
                self.topic_control_current.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(current),
            )
            .await?;

        Ok(())
    }

    // --------------------------------------------------------------------------------

    /// Publish booting status
    async fn publish_booting_status(&mut self) -> anyhow::Result<()> {
        // Set status to Booting
        self.status = StatusCode::Booting;

        // Build status payload
        let status_payload = StatusBuilder::default()
            .with_code(self.status.clone())
            .with_message("Power supply is booting".to_string())
            .build()?;

        // Publish status payload
        self.mqtt_client
            .client
            .publish(
                self.topic_status.clone(),
                rumqttc::QoS::AtMostOnce,
                true,
                status_payload.as_bytes().clone(),
            )
            .await?;

        // Validate publish
        Ok(())
    }
    // --------------------------------------------------------------------------------

    /// Publish running status
    async fn publish_running_status(&mut self) -> anyhow::Result<()> {
        // Set status to Running
        self.status = StatusCode::Running;

        // Build status payload
        let status_payload = StatusBuilder::default()
            .with_code(self.status.clone())
            .with_message("Power supply is running".to_string())
            .build()?;

        // Publish status payload
        self.mqtt_client
            .client
            .publish(
                self.topic_status.clone(),
                rumqttc::QoS::AtMostOnce,
                true,
                status_payload.as_bytes().clone(),
            )
            .await?;

        // Validate publish
        Ok(())
    }

    // --------------------------------------------------------------------------------

    /// Publish panicking status
    async fn publish_panicking_status(&mut self, error_message: String) -> anyhow::Result<()> {
        // Set status to Panicking
        self.status = StatusCode::Panicking;

        // Build status payload
        let status_payload = StatusBuilder::default()
            .with_code(self.status.clone())
            .with_message(error_message)
            .build()?;

        // Publish status payload
        self.mqtt_client
            .client
            .publish(
                self.topic_status.clone(),
                rumqttc::QoS::AtMostOnce,
                true,
                status_payload.as_bytes().clone(),
            )
            .await?;

        // Validate publish
        Ok(())
    }

    // --------------------------------------------------------------------------------

    /// Handle incoming MQTT messages
    ///
    /// This function must manage error handling internally, updating error topics.
    /// If this returns an error, it indicates a critical failure in message handling
    /// and the runner should pass the instance into 'Panic' status.
    ///
    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) -> anyhow::Result<()> {
        // Extract suffix from topic
        let suffix = topic
            .strip_prefix(&self.topic_base)
            .ok_or_else(|| anyhow!("Failed to extract suffix from topic: {}", topic))?;

        // Match the suffix to determine the command
        let cmd = CommandHandler::from_str(suffix);

        // Handle commands based on the topic
        match cmd {
            Some(CommandHandler::OutputSet) => {
                self.handle_output_enable_command(payload).await;
            }
            Some(CommandHandler::VoltageSet) => {
                self.handle_voltage_command(payload).await;
            }
            Some(CommandHandler::CurrentSet) => {
                self.handle_current_command(payload).await;
            }
            None => {
                // Unknown command
                warn!(
                    "[{}] Unknown command for topic suffix: {}",
                    self.name, suffix
                );
            }
        }

        // Acknowledge the command
        Ok(())
    }

    // --------------------------------------------------------------------------------

    /// Handle output enable/disable commands
    async fn handle_output_enable_command(&self, payload: Bytes) {
        // Handle ON/OFF payload
        let cmd = String::from_utf8(payload.to_vec()).unwrap();
        let mut driver = self.driver.lock().await;
        if cmd == "ON" {
            driver
                .enable_output()
                .await
                .expect("Failed to enable output");
        } else if cmd == "OFF" {
            driver
                .disable_output()
                .await
                .expect("Failed to disable output");
        } else {
            // Invalid command
            self.mqtt_client
                .client
                .publish(
                    self.topic_control_oe.clone(),
                    rumqttc::QoS::AtLeastOnce,
                    true,
                    Bytes::from("ERROR"),
                )
                .await
                .unwrap();
            return;
        }

        // Read back the actual output enable state to confirm
        let oe_value = driver.output_enabled().await.expect("Failed to get state");
        let payload_back = Bytes::from(if oe_value { "ON" } else { "OFF" });

        // Confirm the new state by publishing it
        self.mqtt_client
            .client
            .publish(
                self.topic_control_oe.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                payload_back,
            )
            .await
            .unwrap();
    }

    // --------------------------------------------------------------------------------

    /// Handle voltage setting commands
    async fn handle_voltage_command(&self, payload: Bytes) {
        let cmd = String::from_utf8(payload.to_vec()).unwrap();
        let mut driver = self.driver.lock().await;
        driver
            .set_voltage(cmd)
            .await
            .expect("Failed to set voltage");

        // Read back the actual set voltage to confirm
        let voltage = driver.get_voltage().await.expect("Failed to get voltage");
        let payload_back = Bytes::from(voltage);

        // Confirm the new state by publishing it
        self.mqtt_client
            .client
            .publish(
                self.topic_control_voltage.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                payload_back,
            )
            .await
            .unwrap();
    }

    // --------------------------------------------------------------------------------

    /// Handle current setting commands
    async fn handle_current_command(&self, payload: Bytes) {
        let cmd = String::from_utf8(payload.to_vec()).unwrap();
        let mut driver = self.driver.lock().await;
        driver
            .set_current(cmd)
            .await
            .expect("Failed to set current");

        // Confirm the new state by publishing it
        self.mqtt_client
            .client
            .publish(
                self.topic_control_current.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                payload,
            )
            .await
            .unwrap();
    }
}
