use crate::config::GlobalConfig;
use crate::config::MqttBrokerConfig;
use bytes::Bytes;
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;

mod error;
pub use error::ClientError;

fn generate_random_string(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

/// Generate MQTT topic for a given power supply and suffix
///
fn psu_topic<A: Into<String>, B: Into<String>>(name: A, suffix: B) -> String {
    format!("power-supply/{}/{}", name.into(), suffix.into())
}

pub struct ClientBuilder {
    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,
}

impl ClientBuilder {
    pub fn from_user_config_file() -> Self {
        Self {
            broker: GlobalConfig::from_user_file().broker,
        }
    }

    pub fn from_broker_config(broker: MqttBrokerConfig) -> Self {
        Self { broker }
    }

    pub fn build(self) -> Client {
        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            self.broker.host,
            self.broker.port,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        Client::new_with_client(client, event_loop)
    }
}

#[derive(Clone)]
pub struct Client {
    /// MQTT client
    mqtt_client: AsyncClient,
}

impl Client {
    pub fn new_with_client(client: AsyncClient, mut event_loop: rumqttc::EventLoop) -> Self {
        let _task_handler = tokio::spawn(async move {
            // // Subscribe to all relevant topics
            // Self::subscribe_to_all(
            //     client.clone(),
            //     vec![
            //         &runner.topic_control_oe_cmd,
            //         &runner.topic_control_voltage_cmd,
            //         &runner.topic_control_current_cmd,
            //         &runner.topic_measure_voltage_refresh_freq,
            //         &runner.topic_measure_current_refresh_freq,
            //     ],
            // )
            // .await;

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

                                    // runner.handle_incoming_message(&topic, payload).await;
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
                                _ => {}
                            }
                        } // }
                          // }
                          // Err(_) => todo!(),
                    }
                }
            }
        });

        Self {
            mqtt_client: client,
        }
    }

    /// Publish a message to a topic
    ///
    pub async fn publish<A: Into<String>>(
        &self,
        topic: A,
        payload: Bytes,
    ) -> Result<(), rumqttc::ClientError> {
        self.mqtt_client
            .publish(topic.into(), rumqttc::QoS::AtLeastOnce, false, payload)
            .await
    }

    /// Get a PowerSupplyClient for a specific power supply unit
    ///
    pub fn get_power_supply_client(&self, psu_name: String) -> PowerSupplyClient {
        PowerSupplyClient::new(psu_name, self.clone())
    }
}

#[derive(Clone)]
pub struct PowerSupplyClient {
    psu_name: String,

    client: Client,

    /// psu/{name}/control/oe
    topic_control_oe: String,

    /// psu/{name}/control/oe/cmd
    topic_control_oe_cmd: String,

    topic_control_oe_error: String,

    /// psu/{name}/control/voltage/cmd
    topic_control_voltage_cmd: String,

    /// psu/{name}/control/current/cmd
    topic_control_current_cmd: String,

    /// psu/{name}/measure/voltage/refresh_freq
    topic_measure_voltage_refresh_freq: String,

    /// psu/{name}/measure/current/refresh_freq
    topic_measure_current_refresh_freq: String,
}

impl PowerSupplyClient {
    pub fn new(psu_name: String, client: Client) -> Self {
        // Prepare MQTT topics
        let topic_control_oe = psu_topic(psu_name.clone(), "control/oe");
        let topic_control_oe_cmd = psu_topic(psu_name.clone(), "control/oe/cmd");
        let topic_control_oe_error = psu_topic(psu_name.clone(), "control/oe/error");
        let topic_control_voltage_cmd = psu_topic(psu_name.clone(), "control/voltage/cmd");
        let topic_control_current_cmd = psu_topic(psu_name.clone(), "control/current/cmd");
        let topic_measure_voltage_refresh_freq =
            psu_topic(psu_name.clone(), "measure/voltage/refresh_freq");
        let topic_measure_current_refresh_freq =
            psu_topic(psu_name.clone(), "measure/current/refresh_freq");

        Self {
            psu_name,
            client,
            topic_control_oe,
            topic_control_oe_cmd,
            topic_control_oe_error,
            topic_control_voltage_cmd,
            topic_control_current_cmd,
            topic_measure_voltage_refresh_freq,
            topic_measure_current_refresh_freq,
        }
    }

    /// Enable the power supply output
    ///
    pub async fn enable_output(&self) -> Result<(), ClientError> {
        let payload = Bytes::from("ON");
        if let Err(e) = self
            .client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    /// Disable the power supply output
    ///
    pub async fn disable_output(&self) -> Result<(), ClientError> {
        let payload = Bytes::from("OFF");
        if let Err(e) = self
            .client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    pub async fn set_voltage(&self, voltage: String) -> Result<(), ClientError> {
        let payload = Bytes::from(voltage);
        if let Err(e) = self
            .client
            .publish(self.topic_control_voltage_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    pub async fn set_current(&self, current: String) -> Result<(), ClientError> {
        let payload = Bytes::from(current);
        if let Err(e) = self
            .client
            .publish(self.topic_control_current_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }
}
