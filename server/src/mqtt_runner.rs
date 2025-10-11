use crate::drivers::PowerSupplyDriver;
use bytes::Bytes;
use rumqttc::{AsyncClient, MqttOptions};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub mod helper;
use helper::{generate_random_string, psu_topic};

/// Handler for the MQTT Runner task
pub struct RunnerHandler {
    /// Task handler
    pub task_handler: tokio::task::JoinHandle<()>,
}

/// MQTT Runner for handling power supply commands and measurements
pub struct Runner {
    /// MQTT client
    client: AsyncClient,
    /// Instance name
    name: String,

    /// Driver instance
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

impl Runner {
    // --------------------------------------------------------------------------------

    /// Start the runner
    pub fn start(
        name: String,
        driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) -> RunnerHandler {
        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            "localhost",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));
        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        // Create runner object
        let runner = Runner {
            client: client.clone(),
            name: name.clone(),
            driver,
            topic_status: psu_topic(&name, "status"),
            topic_error: psu_topic(&name, "error"),
            topic_control_oe: psu_topic(&name, "control/oe"),
            topic_control_oe_cmd: psu_topic(&name, "control/oe/cmd"),
            topic_control_voltage: psu_topic(&name, "control/voltage"),
            topic_control_voltage_cmd: psu_topic(&name, "control/voltage/cmd"),
            topic_control_current: psu_topic(&name, "control/current"),
            topic_control_current_cmd: psu_topic(&name, "control/current/cmd"),
            topic_measure_voltage_refresh_freq: psu_topic(&name, "measure/voltage/refresh_freq"),
            topic_measure_current_refresh_freq: psu_topic(&name, "measure/current/refresh_freq"),
        };

        let task_handler = tokio::spawn(Self::task_loop(client.clone(), event_loop, runner));

        RunnerHandler { task_handler }
    }

    // --------------------------------------------------------------------------------

    /// The main async task loop for the MQTT runner
    async fn task_loop(client: AsyncClient, mut event_loop: rumqttc::EventLoop, runner: Runner) {
        // Subscribe to all relevant topics
        Self::subscribe_to_all(
            client.clone(),
            vec![
                &runner.topic_control_oe_cmd,
                &runner.topic_control_voltage_cmd,
                &runner.topic_control_current_cmd,
                &runner.topic_measure_voltage_refresh_freq,
                &runner.topic_measure_current_refresh_freq,
            ],
        )
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

    /// Subscribe to all relevant MQTT topics
    async fn subscribe_to_all(client: AsyncClient, topics: Vec<&String>) {
        for topic in topics {
            client
                .subscribe(topic, rumqttc::QoS::AtMostOnce)
                .await
                .unwrap();
        }
    }

    // --------------------------------------------------------------------------------

    /// Initialize the runner (if needed)
    async fn initialize(&self) {
        let mut driver = self.driver.lock().await;

        driver.initialize().await.expect("Driver init failed");

        // Publish initial output enable state
        let oe_value = driver.output_enabled().await.unwrap();
        self.client
            .publish(
                self.topic_control_oe.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(if oe_value { "ON" } else { "OFF" }),
            )
            .await
            .unwrap();

        // Get and check initial voltage setting
        let mut voltage = driver.get_voltage().await.unwrap();
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
            .publish(
                self.topic_control_voltage.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(voltage),
            )
            .await
            .unwrap();

        // Get and check initial current setting
        let mut current = driver.get_current().await.unwrap();
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
            .publish(
                self.topic_control_current.clone(),
                rumqttc::QoS::AtLeastOnce,
                true,
                Bytes::from(current),
            )
            .await
            .unwrap();
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
