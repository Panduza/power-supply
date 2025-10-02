use crate::config::GlobalConfig;
use crate::config::MqttBrokerConfig;
use bytes::Bytes;
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

mod data;
pub use data::MutableData;

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

pub struct PowerSupplyClientBuilder {
    /// Name of the power supply unit
    pub psu_name: Option<String>,

    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,
}

impl PowerSupplyClientBuilder {
    pub fn from_user_config_file() -> Self {
        Self {
            psu_name: None,
            broker: GlobalConfig::from_user_file().broker,
        }
    }

    pub fn from_broker_config(broker: MqttBrokerConfig) -> Self {
        Self {
            psu_name: None,
            broker,
        }
    }

    pub fn with_power_supply_name<A: Into<String>>(mut self, name: A) -> Self {
        self.psu_name = Some(name.into());
        self
    }

    pub fn build(self) -> PowerSupplyClient {
        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            self.broker.host,
            self.broker.port,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        PowerSupplyClient::new_with_client(self.psu_name.unwrap(), client, event_loop)
    }
}

#[derive(Clone)]
pub struct PowerSupplyClient {
    psu_name: String,

    mqtt_client: AsyncClient,

    mutable_data: Arc<Mutex<MutableData>>,

    /// psu/{name}/control/oe
    topic_control_oe: String,

    /// psu/{name}/control/oe/cmd
    topic_control_oe_cmd: String,

    topic_control_oe_error: String,

    /// psu/{name}/control/voltage
    topic_control_voltage: String,

    /// psu/{name}/control/voltage/cmd
    topic_control_voltage_cmd: String,

    /// psu/{name}/control/current
    topic_control_current: String,

    /// psu/{name}/control/current/cmd
    topic_control_current_cmd: String,

    /// psu/{name}/measure/voltage/refresh_freq
    topic_measure_voltage_refresh_freq: String,

    /// psu/{name}/measure/current/refresh_freq
    topic_measure_current_refresh_freq: String,
}

impl PowerSupplyClient {
    /// Task loop to handle MQTT events
    async fn task_loop(client: PowerSupplyClient, mut event_loop: rumqttc::EventLoop) {
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

                                client.handle_incoming_message(&topic, payload).await;
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
    }

    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) {
        if topic == &self.topic_control_oe {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let enabled = msg.trim().eq_ignore_ascii_case("ON");
            let mut data = self.mutable_data.lock().await;
            data.enabled = enabled;
        } else if topic == &self.topic_control_voltage {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let mut data = self.mutable_data.lock().await;
            data.voltage = msg.trim().to_string();
        } else if topic == &self.topic_control_current {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let mut data = self.mutable_data.lock().await;
            data.current = msg.trim().to_string();
        }
    }

    ///
    ///
    pub fn new_with_client(
        psu_name: String,
        client: AsyncClient,
        event_loop: rumqttc::EventLoop,
    ) -> Self {
        // Prepare MQTT topics
        let topic_control_oe = psu_topic(psu_name.clone(), "control/oe");
        let topic_control_oe_cmd = psu_topic(psu_name.clone(), "control/oe/cmd");
        let topic_control_oe_error = psu_topic(psu_name.clone(), "control/oe/error");
        let topic_control_voltage = psu_topic(psu_name.clone(), "control/voltage");
        let topic_control_voltage_cmd = psu_topic(psu_name.clone(), "control/voltage/cmd");
        let topic_control_current = psu_topic(psu_name.clone(), "control/current");
        let topic_control_current_cmd = psu_topic(psu_name.clone(), "control/current/cmd");
        let topic_measure_voltage_refresh_freq =
            psu_topic(psu_name.clone(), "measure/voltage/refresh_freq");
        let topic_measure_current_refresh_freq =
            psu_topic(psu_name.clone(), "measure/current/refresh_freq");

        let obj = Self {
            psu_name,
            mqtt_client: client,

            mutable_data: Arc::new(Mutex::new(MutableData::default())),

            topic_control_oe,
            topic_control_oe_cmd,
            topic_control_oe_error,
            topic_control_voltage,
            topic_control_voltage_cmd,
            topic_control_current,
            topic_control_current_cmd,
            topic_measure_voltage_refresh_freq,
            topic_measure_current_refresh_freq,
        };

        let _task_handler = tokio::spawn(Self::task_loop(obj.clone(), event_loop));
        obj
    }

    pub async fn get_oe(&self) -> bool {
        self.mutable_data.lock().await.enabled
    }

    pub async fn get_voltage(&self) -> String {
        self.mutable_data.lock().await.voltage.clone()
    }

    pub async fn get_current(&self) -> String {
        self.mutable_data.lock().await.current.clone()
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

    /// Enable the power supply output
    ///
    pub async fn enable_output(&self) -> Result<(), ClientError> {
        let payload = Bytes::from("ON");
        if let Err(e) = self
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
            .publish(self.topic_control_current_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }
}
