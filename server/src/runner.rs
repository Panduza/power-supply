use crate::{drivers::PowerSupplyDriver, runner};
use bytes::Bytes;
use rand::{distributions::Alphanumeric, Rng};
use rumqttc::{AsyncClient, MqttOptions};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

pub struct RunnerHandler {
    task_handler: tokio::task::JoinHandle<()>,
}

pub struct Runner {
    /// MQTT client
    client: AsyncClient,

    /// Instance name
    name: String,

    /// Driver instance
    driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,

    topic_control_oe: String,

    /// psu/{name}/control/oe/cmd"
    topic_control_oe_cmd: String,

    /// psu/{name}/control/voltage/cmd
    topic_control_voltage_cmd: String,

    /// psu/{name}/control/current/cmd
    topic_control_current_cmd: String,

    /// psu/{name}/measure/voltage/refresh_freq
    topic_measure_voltage_refresh_freq: String,

    /// psu/{name}/measure/current/refresh_freq
    topic_measure_current_refresh_freq: String,
}

impl Runner {
    /// Generate MQTT topic for a given power supply and suffix
    ///
    fn psu_topic<A: Into<String>, B: Into<String>>(name: A, suffix: B) -> String {
        format!("psu/{}/{}", name.into(), suffix.into())
    }

    /// Subscribe to all relevant MQTT topics
    async fn subscribe_to_all(client: AsyncClient, topics: Vec<&String>) {
        for topic in topics {
            client
                .subscribe(topic, rumqttc::QoS::AtMostOnce)
                .await
                .unwrap();
        }
    }

    /// Start the runner
    ///
    pub fn start(
        name: String,
        driver: Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) -> RunnerHandler {
        // Prepare MQTT topics
        let topic_control_oe = Self::psu_topic(&name, "control/oe");
        let topic_control_oe_cmd = Self::psu_topic(&name, "control/oe/cmd");
        let topic_control_oe_error = Self::psu_topic(&name, "control/oe/error");
        let topic_control_voltage_cmd = Self::psu_topic(&name, "control/voltage/cmd");
        let topic_control_current_cmd = Self::psu_topic(&name, "control/current/cmd");
        let topic_measure_voltage_refresh_freq =
            Self::psu_topic(&name, "measure/voltage/refresh_freq");
        let topic_measure_current_refresh_freq =
            Self::psu_topic(&name, "measure/current/refresh_freq");

        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            "localhost",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, mut event_loop) = AsyncClient::new(mqttoptions, 100);

        // Create runner object
        let runner = Runner {
            client: client.clone(),
            name,
            driver,
            topic_control_oe,
            topic_control_oe_cmd,
            topic_control_voltage_cmd,
            topic_control_current_cmd,
            topic_measure_voltage_refresh_freq,
            topic_measure_current_refresh_freq,
        };

        let task_handler = tokio::spawn(async move {
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
                    // println!("Notification = {:?}", event);
                    // match notification {
                    //     Ok(event) => {
                    match event {
                        rumqttc::Event::Incoming(incoming) => {
                            // println!("Incoming = {:?}", incoming);

                            match incoming {
                                // rumqttc::Packet::Connect(_) => todo!(),
                                // rumqttc::Packet::ConnAck(_) => todo!(),
                                rumqttc::Packet::Publish(packet) => {
                                    // println!("Publish = {:?}", packet);
                                    let topic = packet.topic;
                                    let payload = packet.payload;

                                    runner.handle_incoming_message(&topic, payload).await;
                                }

                                _ => {}
                            }
                        }
                        rumqttc::Event::Outgoing(outgoing) => {
                            // println!("Outgoing = {:?}", outgoing);
                            match outgoing {
                                // rumqttc::Outgoing::Publish(packet) => {
                                //     // println!("Publish = {:?}", packet);
                                // }
                                // rumqttc::Outgoing::Subscribe(p) => {
                                //     // println!("Subscribe = {:?}", p);
                                // }
                                _ => {}
                            }
                        } // }
                          // }
                          // Err(_) => todo!(),
                    }
                }
            }
        });

        println!("MESSAGE ENGINE STOP !! ");

        RunnerHandler { task_handler }
    }

    /// Initialize the runner (if needed)
    async fn initialize(&self) {
        let mut driver = self.driver.lock().await;

        let oe_value = driver.output_enabled().await.unwrap();

        self.client
            .publish(
                self.topic_control_oe.clone(),
                rumqttc::QoS::AtLeastOnce,
                false,
                Bytes::from(if oe_value { "ON" } else { "OFF" }),
            )
            .await
            .unwrap();
    }

    ///
    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) {
        // ----------------------------------------------------------
        // ON/OFF Output Enable
        if topic.eq(&self.topic_control_oe_cmd) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if cmd == "ON" {
                let mut driver = self.driver.lock().await;
                driver
                    .enable_output()
                    .await
                    .expect("Failed to enable output");
            } else if cmd == "OFF" {
                let mut driver = self.driver.lock().await;
                driver
                    .disable_output()
                    .await
                    .expect("Failed to disable output");
            }
        // ----------------------------------------------------------
        // Set Voltage
        } else if topic.eq(&self.topic_control_voltage_cmd) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(voltage) = cmd.parse::<f32>() {
                let mut driver = self.driver.lock().await;
                // driver.set_voltage(voltage).unwrap();
            }
        } else if topic.eq(&self.topic_control_current_cmd) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(current) = cmd.parse::<f32>() {
                let mut driver = self.driver.lock().await;
                // driver.set_current(current).unwrap();
            }
        } else if topic.eq(&self.topic_measure_voltage_refresh_freq) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(freq) = cmd.parse::<u64>() {
                // Set voltage measurement refresh frequency
                // (Implementation depends on the driver capabilities)
            }
        } else if topic.eq(&self.topic_measure_current_refresh_freq) {
            let cmd = String::from_utf8(payload.to_vec()).unwrap();
            if let Ok(freq) = cmd.parse::<u64>() {
                // Set current measurement refresh frequency
                // (Implementation depends on the driver capabilities)
            }
        }

        //         ,
        // "control/voltage/cmd",
        // "control/current/cmd",
        // "measure/voltage/refresh_freq",
        // "measure/current/refresh_freq",

        // psu/{Name}/control/oe
        // psu/{Name}/control/oe/cmd
        // psu/{Name}/control/voltage
        // psu/{Name}/control/voltage/cmd
        // psu/{Name}/control/current
        // psu/{Name}/control/current/cmd

        // psu/{Name}/measure/voltage
        // psu/{Name}/measure/voltage/refresh_freq
        // psu/{Name}/measure/current
        // psu/{Name}/measure/current/refresh_freq
    }
}
