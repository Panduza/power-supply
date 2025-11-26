use bytes::Bytes;
use rumqttc::{AsyncClient, MqttOptions};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing::trace;

mod builder;
pub use builder::PowerSupplyClientBuilder;

mod data;
pub use data::MutableData;

mod error;
pub use error::ClientError;

use crate::payload::PowerStatePayload;

use pza_toolkit::rumqtt::client::RumqttCustomAsyncClient;

/// Client for interacting with a power supply via MQTT
pub struct PowerSupplyClient {
    pub psu_name: String,

    /// The underlying MQTT client
    mqtt_client: RumqttCustomAsyncClient,

    mutable_data: Arc<Mutex<MutableData>>,

    /// Channel for output enable state changes
    oe_channel: (broadcast::Sender<bool>, broadcast::Receiver<bool>),
    /// Channel for voltage changes
    voltage_channel: (broadcast::Sender<String>, broadcast::Receiver<String>),
    /// Channel for current changes
    current_channel: (broadcast::Sender<String>, broadcast::Receiver<String>),
}

impl Clone for PowerSupplyClient {
    fn clone(&self) -> Self {
        Self {
            psu_name: self.psu_name.clone(),
            mqtt_client: self.mqtt_client.clone(),
            mutable_data: Arc::clone(&self.mutable_data),
            oe_channel: (self.oe_channel.0.clone(), self.oe_channel.1.resubscribe()),
            voltage_channel: (
                self.voltage_channel.0.clone(),
                self.voltage_channel.1.resubscribe(),
            ),
            current_channel: (
                self.current_channel.0.clone(),
                self.current_channel.1.resubscribe(),
            ),
            topic_control_oe: self.topic_control_oe.clone(),
            topic_control_oe_cmd: self.topic_control_oe_cmd.clone(),
            topic_control_voltage: self.topic_control_voltage.clone(),
            topic_control_voltage_cmd: self.topic_control_voltage_cmd.clone(),
            topic_control_current: self.topic_control_current.clone(),
            topic_control_current_cmd: self.topic_control_current_cmd.clone(),
        }
    }
}

impl PowerSupplyClient {
    /// Create a new builder for the PowerSupplyClient
    pub fn builder() -> PowerSupplyClientBuilder {
        PowerSupplyClientBuilder::default()
    }

    /// Task loop to handle MQTT events and update client state
    async fn task_loop(
        client: PowerSupplyClient,
        mut event_loop: rumqttc::EventLoop,
        sub_topics: Vec<String>,
    ) {
        // Subscribe to all relevant topics
        client
            .mqtt_client
            .subscribe_to_all(sub_topics.clone())
            .await;

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

    // ------------------------------------------------------------------------

    /// Handle incoming MQTT messages and update internal state
    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) {
        if topic == &self.topic_control_oe {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let enabled = msg.trim().eq_ignore_ascii_case("ON");

            // Update internal state
            {
                let mut data = self.mutable_data.lock().await;
                data.enabled = enabled;
            }

            // Broadcast to all listeners
            self.oe_channel.0.send(enabled).expect("channel error");
        } else if topic == &self.topic_control_voltage {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let voltage_str = msg.trim().to_string();

            // Update internal state
            {
                let mut data = self.mutable_data.lock().await;
                data.voltage = voltage_str.clone();
            }

            self.voltage_channel
                .0
                .send(voltage_str.clone())
                .expect("channel error");
        } else if topic == &self.topic_control_current {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let current_str = msg.trim().to_string();

            // Update internal state
            {
                let mut data = self.mutable_data.lock().await;
                data.current = current_str.clone();
            }

            self.current_channel
                .0
                .send(current_str.clone())
                .expect("channel error");
        }
    }

    // ------------------------------------------------------------------------

    /// Create a new PowerSupplyClient with existing MQTT client and event loop
    pub fn new_with_client(
        psu_name: String,
        client: RumqttCustomAsyncClient,
        event_loop: rumqttc::EventLoop,
    ) -> Self {
        // Prepare MQTT topics
        let topic_control_oe = client.topic_with_prefix("control/oe");
        let topic_control_oe_cmd = client.topic_with_prefix("control/oe/cmd");
        // let topic_control_oe_error = psu_topic(psu_name.clone(), "control/oe/error");
        let topic_control_voltage = client.topic_with_prefix("control/voltage");
        let topic_control_voltage_cmd = client.topic_with_prefix("control/voltage/cmd");
        let topic_control_current = client.topic_with_prefix("control/current");
        let topic_control_current_cmd = client.topic_with_prefix("control/current/cmd");
        // let topic_measure_voltage_refresh_freq =
        //     psu_topic(psu_name.clone(), "measure/voltage/refresh_freq");
        // let topic_measure_current_refresh_freq =
        //     psu_topic(psu_name.clone(), "measure/current/refresh_freq");

        let (oe_tx, oe_rx) = broadcast::channel(32);

        let obj = Self {
            psu_name,
            mqtt_client: client,

            mutable_data: Arc::new(Mutex::new(MutableData::default())),

            oe_channel: (oe_tx, oe_rx),
            voltage_channel: broadcast::channel(32),
            current_channel: broadcast::channel(32),

            topic_control_oe,
            topic_control_oe_cmd,
            // topic_control_oe_error,
            topic_control_voltage,
            topic_control_voltage_cmd,
            topic_control_current,
            topic_control_current_cmd,
            // topic_measure_voltage_refresh_freq,
            // topic_measure_current_refresh_freq,
        };

        let _task_handler = tokio::spawn(Self::task_loop(
            obj.clone(),
            event_loop,
            vec![
                obj.topic_control_oe.clone(),
                obj.topic_control_voltage.clone(),
                obj.topic_control_current.clone(),
            ],
        ));
        obj
    }

    // ------------------------------------------------------------------------

    /// Get the current output enable state
    pub async fn get_oe(&self) -> bool {
        self.mutable_data.lock().await.enabled
    }

    // ------------------------------------------------------------------------

    /// Get the current voltage setting
    pub async fn get_voltage(&self) -> String {
        self.mutable_data.lock().await.voltage.clone()
    }

    // ------------------------------------------------------------------------

    /// Get the current current setting
    pub async fn get_current(&self) -> String {
        self.mutable_data.lock().await.current.clone()
    }

    // ------------------------------------------------------------------------

    /// Enable the power supply output
    pub async fn enable_output(&self) -> anyhow::Result<()> {
        trace!("[{}] Enabling output", self.psu_name);
        let payload = Bytes::from("ON");
        self.mqtt_client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await?;
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Disable the power supply output
    pub async fn disable_output(&self) -> anyhow::Result<()> {
        trace!("[{}] Disabling output", self.psu_name);
        let payload = Bytes::from("OFF");
        self.mqtt_client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await?;
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Set the voltage of the power supply
    pub async fn set_voltage(&self, voltage: String) -> Result<(), ClientError> {
        let payload = Bytes::from(voltage);
        if let Err(e) = self
            .mqtt_client
            .publish(self.topic_control_voltage_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Set the current limit of the power supply
    pub async fn set_current(&self, current: String) -> Result<(), ClientError> {
        let payload = Bytes::from(current);
        if let Err(e) = self
            .mqtt_client
            .publish(self.topic_control_current_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Dynamic Callback Management
    // ------------------------------------------------------------------------

    /// Subscribe to output enable state changes
    pub fn subscribe_oe_changes(&self) -> broadcast::Receiver<bool> {
        self.oe_channel.0.subscribe()
    }

    /// Subscribe to output voltage state changes
    pub fn subscribe_voltage_changes(&self) -> broadcast::Receiver<String> {
        self.voltage_channel.0.subscribe()
    }

    /// Subscribe to output current state changes
    pub fn subscribe_current_changes(&self) -> broadcast::Receiver<String> {
        self.current_channel.0.subscribe()
    }

    pub fn name(&self) -> String {
        self.psu_name.clone()
    }

    // ------------------------------------------------------------------------
}
