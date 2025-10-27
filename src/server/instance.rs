use crate::constants;
use crate::drivers::PowerSupplyDriver;
use bytes::Bytes;
use dioxus::html::form;
use pza_toolkit::rumqtt::client::init_client;
use pza_toolkit::rumqtt::client::RumqttCustomAsyncClient;
use rumqttc::{AsyncClient, MqttOptions};
use std::any;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct InstanceHandler {
    pub task_handler: Arc<Option<tokio::task::JoinHandle<()>>>,
}

/// MQTT InstanceRunner for handling power supply commands and measurements
pub struct InstanceRunner {
    /// MQTT client
    client: RumqttCustomAsyncClient,
    /// InstanceRunner name
    name: String,

    /// Driver instanceRunner
    driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,

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

impl InstanceRunner {
    // --------------------------------------------------------------------------------

    /// Start the runner
    pub fn start(
        name: String,
        driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) -> anyhow::Result<InstanceHandler> {
        let (client, event_loop) = init_client("tttt");

        let custom_client = RumqttCustomAsyncClient::new(
            client,
            rumqttc::QoS::AtMostOnce,
            true,
            format!("{}/{}", constants::MQTT_TOPIC_PREFIX, name),
        );

        // Create runner object
        let runner = InstanceRunner {
            name: name.clone(),
            driver,
            topic_status: custom_client.topic_with_prefix("status"),
            topic_error: custom_client.topic_with_prefix("error"),
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
            client: custom_client,
        };

        let task_handler = tokio::spawn(Self::task_loop(event_loop, runner));

        Ok(InstanceHandler {
            task_handler: Arc::new(Some(task_handler)),
        })
    }

    // --------------------------------------------------------------------------------

    /// The main async task loop for the MQTT runner
    async fn task_loop(mut event_loop: rumqttc::EventLoop, runner: InstanceRunner) {
        // Subscribe to all relevant topics
        runner
            .client
            .subscribe_to_all(vec![
                runner.topic_control_oe_cmd.clone(),
                runner.topic_control_voltage_cmd.clone(),
                runner.topic_control_current_cmd.clone(),
                runner.topic_measure_voltage_refresh_freq.clone(),
                runner.topic_measure_current_refresh_freq.clone(),
            ])
            .await;

        runner.initialize().await;

        loop {
            while let Ok(event) = event_loop.poll().await {
                match event {
                    rumqttc::Event::Incoming(incoming) => match incoming {
                        rumqttc::Packet::Publish(packet) => {
                            let topic = packet.topic;
                            let payload = packet.payload;
                            runner.handle_incoming_message(&topic, payload).await;
                        }
                        _ => {}
                    },
                    rumqttc::Event::Outgoing(_outgoing) => {}
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
        self.client
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

        self.client
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

        self.client
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
            self.client
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

        // Wait a bit for the device to process the command
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Read back the actual output enable state to confirm
        let oe_value = driver.output_enabled().await.expect("Failed to get state");
        let payload_back = Bytes::from(if oe_value { "ON" } else { "OFF" });

        // Confirm the new state by publishing it
        self.client
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

        // Wait a bit for the device to process the command
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Read back the actual set voltage to confirm
        let voltage = driver.get_voltage().await.expect("Failed to get voltage");
        let payload_back = Bytes::from(voltage);

        // Confirm the new state by publishing it
        self.client
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
        self.client
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

    // --------------------------------------------------------------------------------

    /// Handle incoming MQTT messages
    /// TODO => handle error return here
    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) {
        // ON/OFF Output Enable
        if topic.eq(&self.topic_control_oe_cmd) {
            self.handle_output_enable_command(payload).await;
        }
        // Set Voltage
        else if topic.eq(&self.topic_control_voltage_cmd) {
            self.handle_voltage_command(payload).await;
        }
        // Set Current
        else if topic.eq(&self.topic_control_current_cmd) {
            self.handle_current_command(payload).await;
        }
        // Set Measurement Refresh Frequencies
        else if topic.eq(&self.topic_measure_voltage_refresh_freq) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(_freq) = cmd.parse::<u64>() {
                // Set voltage measurement refresh frequency
                // (Implementation depends on the driver capabilities)
            }
        } else if topic.eq(&self.topic_measure_current_refresh_freq) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(_freq) = cmd.parse::<u64>() {
                // Set current measurement refresh frequency
                // (Implementation depends on the driver capabilities)
            }
        }
    }
}
