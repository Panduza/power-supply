use crate::constants;
use crate::server::drivers::PowerSupplyDriver;
use bytes::Bytes;
use pza_power_supply_client::payload::PowerState;
use pza_power_supply_client::payload::PowerStatePayload;
use pza_power_supply_client::topics::TopicId;
use pza_power_supply_client::topics::Topics;
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
    /// MQTT client
    client: RumqttCustomAsyncClient,
    /// MqttRunner name
    name: String,

    /// Driver MqttRunner
    driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,

    /// MQTT topics used by the runner
    topics: Topics,
}

impl MqttRunner {
    // --------------------------------------------------------------------------------

    /// Start the runner
    pub fn start(
        name: String,
        driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) -> anyhow::Result<MqttRunnerHandler> {
        let (client, event_loop) = init_client("tttt");

        let custom_client = RumqttCustomAsyncClient::new(
            client,
            rumqttc::QoS::AtMostOnce,
            true,
            format!("{}/{}", constants::MQTT_TOPIC_PREFIX, name),
        );

        // Create runner object
        let runner = MqttRunner {
            topics: Topics::new(&name),
            name: name.clone(),
            driver,
            client: custom_client,
        };

        let task_handler = tokio::spawn(Self::task_loop(event_loop, runner));

        Ok(MqttRunnerHandler {
            task_handler: Arc::new(Some(task_handler)),
        })
    }

    // --------------------------------------------------------------------------------

    /// The main async task loop for the MQTT runner
    async fn task_loop(mut event_loop: rumqttc::EventLoop, runner: MqttRunner) {
        // Subscribe to all relevant topics
        runner
            .client
            .subscribe_to_all(runner.topics.vec_sub_server())
            .await;

        runner.initialize().await;

        loop {
            while let Ok(event) = event_loop.poll().await {
                match event {
                    rumqttc::Event::Incoming(incoming) => match incoming {
                        rumqttc::Packet::Publish(packet) => {
                            let topic = packet.topic;
                            let payload = packet.payload;
                            trace!("[{}] Received message on topic: {}", runner.name, topic);
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
                self.topics.state.clone(),
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
                self.topics.voltage.clone(),
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
                self.topics.current.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(current),
            )
            .await?;

        Ok(())
    }

    // --------------------------------------------------------------------------------

    /// Handle output enable/disable commands
    async fn handle_state_command(&self, payload: Bytes) -> anyhow::Result<()> {
        // Deserialize the command payload
        let cmd = PowerStatePayload::from_json_bytes(payload)?;
        trace!("[{}] Handling state command: {:?}", self.name, cmd.state);

        // Handle ON/OFF payload
        let mut driver = self.driver.lock().await;
        if cmd.state == PowerState::On {
            driver.enable_output().await?;
        } else if cmd.state == PowerState::Off {
            driver.disable_output().await?;
        }

        // Read back the actual output enable state to confirm
        let oe_value = driver.output_enabled().await?;
        let payload_back = PowerStatePayload::from_state_as_response(
            if oe_value {
                PowerState::On
            } else {
                PowerState::Off
            },
            cmd.pza_id,
        )
        .to_json_bytes()?;

        // Confirm the new state by publishing it
        self.client
            .pubsh(&self.topics.state, payload_back)
            .await
            .unwrap();
        Ok(())
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
        self.client
            .client
            .publish(
                self.topics.voltage.clone(),
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
                self.topics.current.clone(),
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
        let id = self.topics.topic_to_id(topic);

        match id {
            Some(TopicId::StateCmd) => {
                self.handle_state_command(payload).await;
            }
            Some(TopicId::VoltageCmd) => {
                self.handle_voltage_command(payload).await;
            }
            Some(TopicId::CurrentCmd) => {
                self.handle_current_command(payload).await;
            }
            _ => {
                // Unknown or unhandled topic
                trace!(
                    "[{}] Received message on unhandled topic: {}",
                    self.name,
                    topic
                );
            }
        }
    }
}
