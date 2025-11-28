use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing::error;
use tracing::trace;
use tracing::warn;

mod builder;
pub use builder::PowerSupplyClientBuilder;

mod data;
pub use data::MutableData;

mod error;
pub use error::ClientError;

use crate::payload::CurrentPayload;
use crate::payload::PowerState;
use crate::payload::PowerStatePayload;
use crate::payload::VoltagePayload;
use crate::topics::Topics;

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

    /// MQTT topics used by the client
    topics: Topics,
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
            topics: self.topics.clone(),
        }
    }
}

impl PowerSupplyClient {
    /// Create a new builder for the PowerSupplyClient
    pub fn builder() -> PowerSupplyClientBuilder {
        PowerSupplyClientBuilder::default()
    }

    /// Task loop to handle MQTT events and update client state
    async fn task_loop(client: PowerSupplyClient, mut event_loop: rumqttc::EventLoop) {
        // Subscribe to all relevant topics
        client
            .mqtt_client
            .subscribe_to_all(client.topics.vec_sub_client())
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
        let id = self.topics.topic_to_id(topic);

        match id {
            None => {
                error!("[{}] Unknown topic received: {}", self.psu_name, topic);
                return;
            }
            Some(crate::topics::TopicId::Status) => {
                // Handle status updates
                trace!("[{}] Status update received", self.psu_name);
            }
            Some(crate::topics::TopicId::Error) => {
                // Handle error messages
                let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
                error!("[{}] Error received: {}", self.psu_name, msg);
            }
            Some(crate::topics::TopicId::State) => {
                // Handle state updates (ON/OFF)
                let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
                let enabled = msg.trim().eq_ignore_ascii_case("ON");

                // Update internal state
                {
                    let mut data = self.mutable_data.lock().await;
                    data.enabled = enabled;
                }

                // Broadcast to all listeners
                self.oe_channel.0.send(enabled).expect("channel error");
            }
            Some(crate::topics::TopicId::Voltage) => {
                // Handle voltage updates
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
            }
            Some(crate::topics::TopicId::Current) => {
                // Handle current updates
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
            Some(crate::topics::TopicId::StateCmd)
            | Some(crate::topics::TopicId::VoltageCmd)
            | Some(crate::topics::TopicId::CurrentCmd) => {
                // These are command topics that the client sends to, not receives from
                warn!(
                    "[{}] Unexpected command topic received: {}",
                    self.psu_name, topic
                );
            }
        }
    }

    // ------------------------------------------------------------------------

    /// Create a new PowerSupplyClient with existing MQTT client and event loop
    pub fn new_with_client(
        psu_name: String,
        client: RumqttCustomAsyncClient,
        event_loop: rumqttc::EventLoop,
    ) -> Self {
        let (oe_tx, oe_rx) = broadcast::channel(32);

        let obj = Self {
            topics: Topics::new(&psu_name),
            psu_name,
            mqtt_client: client,

            mutable_data: Arc::new(Mutex::new(MutableData::default())),

            oe_channel: (oe_tx, oe_rx),
            voltage_channel: broadcast::channel(32),
            current_channel: broadcast::channel(32),
        };

        let _task_handler = tokio::spawn(Self::task_loop(obj.clone(), event_loop));
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
        self.mqtt_client
            .pubsh(
                &self.topics.state_cmd,
                PowerStatePayload::from_state(PowerState::On).to_json_bytes()?,
            )
            .await
    }

    // ------------------------------------------------------------------------

    /// Disable the power supply output
    pub async fn disable_output(&self) -> anyhow::Result<()> {
        trace!("[{}] Disabling output", self.psu_name);
        self.mqtt_client
            .pubsh(
                &self.topics.state_cmd,
                PowerStatePayload::from_state(PowerState::Off).to_json_bytes()?,
            )
            .await
    }

    // ------------------------------------------------------------------------

    /// Set the voltage of the power supply
    pub async fn set_voltage(&self, voltage: String) -> anyhow::Result<()> {
        trace!("[{}] Setting voltage to {}", self.psu_name, voltage);
        self.mqtt_client
            .pubsh(
                &self.topics.voltage_cmd,
                VoltagePayload::new(voltage).to_json_bytes()?,
            )
            .await
    }

    // ------------------------------------------------------------------------

    /// Set the current limit of the power supply
    pub async fn set_current(&self, current: String) -> anyhow::Result<()> {
        trace!("[{}] Setting current to {}", self.psu_name, current);
        self.mqtt_client
            .pubsh(
                &self.topics.current_cmd,
                CurrentPayload::new(current).to_json_bytes()?,
            )
            .await
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
