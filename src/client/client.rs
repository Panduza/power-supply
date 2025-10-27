use bytes::Bytes;
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::Mutex;

mod data;
pub use data::MutableData;

mod error;
pub use error::ClientError;

use pza_toolkit::rumqtt_client::RumqttCustomAsyncClient;
use std::collections::HashMap;

/// Type alias for async callbacks
pub type AsyncCallback<T> =
    Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback ID type for identifying callbacks
pub type CallbackId = u64;

/// Dynamic callbacks structure to hold multiple callbacks per event type
#[derive(Default)]
pub struct DynamicCallbacks {
    pub voltage_callbacks: HashMap<CallbackId, AsyncCallback<String>>,
    pub current_callbacks: HashMap<CallbackId, AsyncCallback<String>>,
    next_id: CallbackId,
}

impl DynamicCallbacks {
    /// Generate a new unique callback ID
    pub fn next_id(&mut self) -> CallbackId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a callback for voltage changes
    pub fn add_voltage_callback(&mut self, callback: AsyncCallback<String>) -> CallbackId {
        let id = self.next_id();
        self.voltage_callbacks.insert(id, callback);
        id
    }

    /// Add a callback for current changes
    pub fn add_current_callback(&mut self, callback: AsyncCallback<String>) -> CallbackId {
        let id = self.next_id();
        self.current_callbacks.insert(id, callback);
        id
    }

    /// Remove a voltage callback
    pub fn remove_voltage_callback(&mut self, id: CallbackId) -> bool {
        self.voltage_callbacks.remove(&id).is_some()
    }

    /// Remove a current callback
    pub fn remove_current_callback(&mut self, id: CallbackId) -> bool {
        self.current_callbacks.remove(&id).is_some()
    }
}

/// Generate a random string of specified length using alphanumeric characters
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
fn psu_topic<A: Into<String>, B: Into<String>>(name: A, suffix: B) -> String {
    format!("power-supply/{}/{}", name.into(), suffix.into())
}

/// Builder pattern for creating PowerSupplyClient instances
pub struct PowerSupplyClientBuilder {
    /// Name of the power supply unit
    pub psu_name: Option<String>,

    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,
}

impl PowerSupplyClientBuilder {
    /// Create a new builder from user configuration file
    pub fn from_user_config_file() -> Self {
        Self {
            psu_name: None,
            broker: GlobalConfig::from_user_file().broker,
        }
    }

    // ------------------------------------------------------------------------

    /// Create a new builder from broker configuration
    pub fn from_broker_config(broker: MqttBrokerConfig) -> Self {
        Self {
            psu_name: None,
            broker,
        }
    }

    // ------------------------------------------------------------------------

    /// Set the power supply name for the client
    pub fn with_power_supply_name<A: Into<String>>(mut self, name: A) -> Self {
        self.psu_name = Some(name.into());
        self
    }

    // ------------------------------------------------------------------------

    /// Build the PowerSupplyClient instance
    pub fn build(self) -> PowerSupplyClient {
        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            self.broker.host,
            self.broker.port,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        PowerSupplyClient::new_with_client(
            self.psu_name.unwrap(),
            RumqttCustomAsyncClient::new(
                client,
                rumqttc::QoS::AtMostOnce,
                true,
                "power-supply".to_string(),
            ),
            event_loop,
        )
    }
}

/// Client for interacting with a power supply via MQTT
pub struct PowerSupplyClient {
    pub psu_name: String,

    /// The underlying MQTT client
    mqtt_client: RumqttCustomAsyncClient,

    mutable_data: Arc<Mutex<MutableData>>,

    callbacks: Arc<Mutex<DynamicCallbacks>>,

    /// Callbacks for output enable state changes
    // oe_callbacks: Arc<Mutex<AsyncCallbackManager<bool>>>,
    /// Channel for output enable state changes
    oe_channel: (broadcast::Sender<bool>, broadcast::Receiver<bool>),

    /// psu/{name}/control/oe
    topic_control_oe: String,
    /// psu/{name}/control/oe/cmd
    topic_control_oe_cmd: String,

    /// psu/{name}/control/voltage
    topic_control_voltage: String,
    /// psu/{name}/control/voltage/cmd
    topic_control_voltage_cmd: String,

    /// psu/{name}/control/current
    topic_control_current: String,
    /// psu/{name}/control/current/cmd
    topic_control_current_cmd: String,
}

impl Clone for PowerSupplyClient {
    fn clone(&self) -> Self {
        Self {
            psu_name: self.psu_name.clone(),
            mqtt_client: self.mqtt_client.clone(),
            mutable_data: Arc::clone(&self.mutable_data),
            callbacks: Arc::clone(&self.callbacks),
            // oe_callbacks: Arc::clone(&self.oe_callbacks),
            oe_channel: (self.oe_channel.0.clone(), self.oe_channel.1.resubscribe()),
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

            // Trigger all voltage callbacks
            let callbacks = self.callbacks.lock().await;
            for callback in callbacks.voltage_callbacks.values() {
                callback(voltage_str.clone()).await;
            }
        } else if topic == &self.topic_control_current {
            let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
            let current_str = msg.trim().to_string();

            // Update internal state
            {
                let mut data = self.mutable_data.lock().await;
                data.current = current_str.clone();
            }

            // Trigger all current callbacks
            let callbacks = self.callbacks.lock().await;
            for callback in callbacks.current_callbacks.values() {
                callback(current_str.clone()).await;
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
        // Prepare MQTT topics
        let topic_control_oe = psu_topic(psu_name.clone(), "control/oe");
        let topic_control_oe_cmd = psu_topic(psu_name.clone(), "control/oe/cmd");
        // let topic_control_oe_error = psu_topic(psu_name.clone(), "control/oe/error");
        let topic_control_voltage = psu_topic(psu_name.clone(), "control/voltage");
        let topic_control_voltage_cmd = psu_topic(psu_name.clone(), "control/voltage/cmd");
        let topic_control_current = psu_topic(psu_name.clone(), "control/current");
        let topic_control_current_cmd = psu_topic(psu_name.clone(), "control/current/cmd");
        // let topic_measure_voltage_refresh_freq =
        //     psu_topic(psu_name.clone(), "measure/voltage/refresh_freq");
        // let topic_measure_current_refresh_freq =
        //     psu_topic(psu_name.clone(), "measure/current/refresh_freq");

        let (oe_tx, oe_rx) = broadcast::channel(32);

        let obj = Self {
            psu_name,
            mqtt_client: client,

            mutable_data: Arc::new(Mutex::new(MutableData::default())),
            callbacks: Arc::new(Mutex::new(DynamicCallbacks::default())),
            // oe_callbacks: Arc::new(Mutex::new(AsyncCallbackManager::new())),
            oe_channel: (oe_tx, oe_rx),

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
    pub async fn enable_output(&self) -> Result<(), ClientError> {
        let payload = Bytes::from("ON");
        if let Err(e) = self
            .mqtt_client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
        Ok(())
    }

    // ------------------------------------------------------------------------

    /// Disable the power supply output
    pub async fn disable_output(&self) -> Result<(), ClientError> {
        let payload = Bytes::from("OFF");
        if let Err(e) = self
            .mqtt_client
            .publish(self.topic_control_oe_cmd.clone(), payload)
            .await
        {
            return Err(ClientError::MqttError(e.to_string()));
        }
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

    // ------------------------------------------------------------------------

    /// Add a callback for voltage changes
    /// Returns the callback ID that can be used to remove it later
    pub async fn add_voltage_callback<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.add_voltage_callback(Box::new(callback))
    }

    // ------------------------------------------------------------------------

    /// Add a callback for current changes
    /// Returns the callback ID that can be used to remove it later
    pub async fn add_current_callback<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.add_current_callback(Box::new(callback))
    }

    // ------------------------------------------------------------------------

    /// Remove a voltage callback by its ID
    /// Returns true if the callback was found and removed
    pub async fn remove_voltage_callback(&self, id: CallbackId) -> bool {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.remove_voltage_callback(id)
    }

    // ------------------------------------------------------------------------

    /// Remove a current callback by its ID
    /// Returns true if the callback was found and removed
    pub async fn remove_current_callback(&self, id: CallbackId) -> bool {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.remove_current_callback(id)
    }

    // ------------------------------------------------------------------------

    /// Helper method to add a simple logging callback for voltage changes
    /// Returns the callback ID
    pub async fn add_voltage_logging(&self) -> CallbackId {
        self.add_voltage_callback(|voltage| {
            Box::pin(async move {
                println!("[PSU] Voltage: {}", voltage);
            })
        })
        .await
    }

    // ------------------------------------------------------------------------

    /// Helper method to add a simple logging callback for current changes
    /// Returns the callback ID
    pub async fn add_current_logging(&self) -> CallbackId {
        self.add_current_callback(|current| {
            Box::pin(async move {
                println!("[PSU] Current: {}", current);
            })
        })
        .await
    }

    // ------------------------------------------------------------------------

    /// Helper method to add logging callbacks for all state changes
    /// Returns a vector of callback IDs
    pub async fn add_all_logging(&self) -> Vec<CallbackId> {
        vec![
            self.add_voltage_logging().await,
            self.add_current_logging().await,
        ]
    }

    // ------------------------------------------------------------------------

    /// Remove all callbacks of all types
    pub async fn clear_all_callbacks(&self) {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.voltage_callbacks.clear();
        callbacks.current_callbacks.clear();
    }

    // ------------------------------------------------------------------------

    /// Subscribe to output enable state changes
    pub fn subscribe_oe_changes(&self) -> broadcast::Receiver<bool> {
        self.oe_channel.0.subscribe()
    }

    // ------------------------------------------------------------------------
}
