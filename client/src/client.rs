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
                                // rumqttc::Packet::PubAck(_) => todo!(),
                                // rumqttc::Packet::PubRec(_) => todo!(),
                                // rumqttc::Packet::PubRel(_) => todo!(),
                                // rumqttc::Packet::PubComp(_) => todo!(),
                                // rumqttc::Packet::Subscribe(_) => todo!(),
                                // rumqttc::Packet::SubAck(_) => todo!(),
                                // rumqttc::Packet::Unsubscribe(_) => todo!(),
                                // rumqttc::Packet::UnsubAck(_) => todo!(),
                                // rumqttc::Packet::PingReq => todo!(),
                                // rumqttc::Packet::PingResp => todo!(),
                                // rumqttc::Packet::Disconnect => todo!(),
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
                                // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
                                // rumqttc::Outgoing::PubAck(_) => todo!(),
                                // rumqttc::Outgoing::PubRec(_) => todo!(),
                                // rumqttc::Outgoing::PubRel(_) => todo!(),
                                // rumqttc::Outgoing::PubComp(_) => todo!(),
                                // rumqttc::Outgoing::PingReq => todo!(),
                                // rumqttc::Outgoing::PingResp => todo!(),
                                // rumqttc::Outgoing::Disconnect => todo!(),
                                // rumqttc::Outgoing::AwaitAck(_) => todo!(),
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
}

pub struct PowerSupplyClient {
    psu_name: String,

    client: Client,
}

impl PowerSupplyClient {
    pub fn new(psu_name: String, client: Client) -> Self {
        Self { psu_name, client }
    }

    /// Enable the power supply output
    ///
    pub async fn enable_output(&self) -> Result<(), ClientError> {
        let topic = format!("psu/{}/control/oe_cmd", self.psu_name);
        let payload = Bytes::from("ON");
        if let Err(e) = self.client.publish(topic, payload).await {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    /// Disable the power supply output
    ///
    pub async fn disable_output(&self) -> Result<(), ClientError> {
        let topic = format!("psu/{}/control/oe_cmd", self.psu_name);
        let payload = Bytes::from("OFF");
        if let Err(e) = self.client.publish(topic, payload).await {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }
}
