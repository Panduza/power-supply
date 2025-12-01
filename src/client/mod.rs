use bytes::Bytes;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio::sync::Mutex;

use tracing::error;
use tracing::trace;
use tracing::warn;

/// Power supply client builder for configuring connection parameters.
mod builder;
use builder::PowerSupplyClientBuilder;

/// Mutable data structures for client state management.
mod data;
pub use data::MutableData;

/// Error types specific to power supply client operations.
mod error;
pub use error::ClientError;

use crate::payload::CurrentPayload;
use crate::payload::PowerState;
use crate::payload::PowerStatePayload;
use crate::payload::PzaId;
use crate::payload::VoltagePayload;
use crate::TopicId;
use crate::Topics;

use pza_toolkit::rumqtt::client::RumqttCustomAsyncClient;

/// Client for interacting with a power supply device via MQTT protocol.
///
/// Provides high-level operations for controlling power supply voltage, current,
/// and output state, with real-time updates via broadcast channels.
pub struct PowerSupplyClient {
    /// Name identifier of the power supply device.
    pub psu_name: String,

    /// The underlying MQTT client for network communication.
    mqtt_client: RumqttCustomAsyncClient,

    /// Thread-safe mutable data container for current device state.
    mutable_data: Arc<Mutex<MutableData>>,

    /// Channel for broadcasting output enable state changes.
    state_channel: (
        broadcast::Sender<Arc<PowerStatePayload>>,
        broadcast::Receiver<Arc<PowerStatePayload>>,
    ),
    /// Channel for broadcasting voltage setting changes.
    voltage_channel: (
        broadcast::Sender<Arc<VoltagePayload>>,
        broadcast::Receiver<Arc<VoltagePayload>>,
    ),
    /// Channel for broadcasting current limit changes.
    current_channel: (
        broadcast::Sender<Arc<CurrentPayload>>,
        broadcast::Receiver<Arc<CurrentPayload>>,
    ),

    /// MQTT topics configuration for this device instance.
    topics: Topics,
}

// ================

impl Clone for PowerSupplyClient {
    // ------------------------------------------------------------------------------

    /// Creates a clone of the PowerSupplyClient with new channel receivers.
    fn clone(&self) -> Self {
        Self {
            psu_name: self.psu_name.clone(),
            mqtt_client: self.mqtt_client.clone(),
            mutable_data: Arc::clone(&self.mutable_data),
            state_channel: (
                self.state_channel.0.clone(),
                self.state_channel.1.resubscribe(),
            ),
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
    // ------------------------------------------------------------------------------
}

// ================

impl PowerSupplyClient {
    // ------------------------------------------------------------------------------

    /// Creates a new builder for configuring PowerSupplyClient instances.
    pub fn builder() -> PowerSupplyClientBuilder {
        PowerSupplyClientBuilder::default()
    }

    // ------------------------------------------------------------------------------

    /// Creates a new PowerSupplyClient with existing MQTT client and event loop.
    ///
    /// Automatically starts the background task loop for handling MQTT events.
    fn new_from_builder(
        psu_name: String,
        client: RumqttCustomAsyncClient,
        event_loop: rumqttc::EventLoop,
    ) -> Self {
        // Initialize broadcast channels for state updates
        let (state_tx, state_rx) = broadcast::channel::<Arc<PowerStatePayload>>(32);
        let (voltage_tx, voltage_rx) = broadcast::channel::<Arc<VoltagePayload>>(32);
        let (current_tx, current_rx) = broadcast::channel::<Arc<CurrentPayload>>(32);

        // Create the client instance
        let obj = Self {
            topics: Topics::new(&psu_name),
            psu_name,
            mqtt_client: client,

            mutable_data: Arc::new(Mutex::new(MutableData::default())),

            state_channel: (state_tx, state_rx),
            voltage_channel: (voltage_tx, voltage_rx),
            current_channel: (current_tx, current_rx),
        };

        // Start the background task loop for handling MQTT events
        let _task_handler = tokio::spawn(Self::task_loop(obj.clone(), event_loop));
        obj
    }

    // ------------------------------------------------------------------------------

    /// Background task loop to handle MQTT events and update client state.
    ///
    /// Subscribes to all relevant topics and processes incoming messages,
    /// updating internal state and broadcasting changes to subscribers.
    async fn task_loop(client: PowerSupplyClient, mut event_loop: rumqttc::EventLoop) {
        // Subscribe to all relevant topics
        client
            .mqtt_client
            .subscribe_to_all(client.topics.vec_sub_client())
            .await;

        // Event loop to process incoming MQTT messages
        loop {
            while let Ok(event) = event_loop.poll().await {
                match event {
                    rumqttc::Event::Incoming(incoming) => match incoming {
                        rumqttc::Packet::Publish(packet) => {
                            let topic = packet.topic;
                            let payload = packet.payload;
                            client.handle_incoming_message(&topic, payload).await;
                        }
                        _ => {}
                    },
                    rumqttc::Event::Outgoing(outgoing) => match outgoing {
                        _ => {}
                    },
                }
            }
        }
    }

    // ------------------------------------------------------------------------------

    /// Handles incoming MQTT messages and updates internal state.
    ///
    /// Processes messages based on topic type and broadcasts updates
    /// to appropriate channels for real-time notifications.
    async fn handle_incoming_message(&self, topic: &String, payload: Bytes) {
        let id = self.topics.topic_to_id(topic);

        match id {
            None => {
                error!("[{}] Unknown topic received: {}", self.psu_name, topic);
                return;
            }
            Some(TopicId::Status) => {
                // Handle status updates
                trace!("[{}] Status update received", self.psu_name);
            }
            Some(TopicId::Error) => {
                // Handle error messages
                let msg = String::from_utf8(payload.to_vec()).unwrap_or_default();
                error!("[{}] Error received: {}", self.psu_name, msg);
            }
            Some(TopicId::State) => {
                // Handle state updates (PowerStatePayload)
                match PowerStatePayload::from_json_bytes(payload) {
                    Ok(state_payload) => {
                        let enabled = state_payload.state == PowerState::On;

                        // Update internal state
                        {
                            let mut data = self.mutable_data.lock().await;
                            data.enabled = enabled;
                        }

                        // Broadcast to all listeners
                        self.state_channel
                            .0
                            .send(Arc::new(state_payload))
                            .expect("channel error");
                    }
                    Err(e) => {
                        error!("[{}] Failed to parse state payload: {}", self.psu_name, e);
                    }
                }
            }
            Some(TopicId::Voltage) => {
                // Handle voltage updates
                match VoltagePayload::from_json_bytes(payload) {
                    Ok(voltage_payload) => {
                        // Update internal state
                        {
                            let mut data = self.mutable_data.lock().await;
                            data.voltage = voltage_payload.voltage.clone();
                        }

                        // Broadcast to all listeners
                        self.voltage_channel
                            .0
                            .send(Arc::new(voltage_payload))
                            .expect("channel error");
                    }
                    Err(e) => {
                        error!("[{}] Failed to parse voltage payload: {}", self.psu_name, e);
                    }
                }
            }
            Some(TopicId::Current) => {
                // Handle current updates
                match CurrentPayload::from_json_bytes(payload) {
                    Ok(current_payload) => {
                        // Update internal state
                        {
                            let mut data = self.mutable_data.lock().await;
                            data.current = current_payload.current.clone();
                        }

                        // Broadcast to all listeners
                        self.current_channel
                            .0
                            .send(Arc::new(current_payload))
                            .expect("channel error");
                    }
                    Err(e) => {
                        error!("[{}] Failed to parse current payload: {}", self.psu_name, e);
                    }
                }
            }
            Some(TopicId::StateCmd) | Some(TopicId::VoltageCmd) | Some(TopicId::CurrentCmd) => {
                // These are command topics that the client sends to, not receives from
                warn!(
                    "[{}] Unexpected command topic received: {}",
                    self.psu_name, topic
                );
            }
        }
    }

    // ------------------------------------------------------------------------------

    /// Gets the current output enable state of the power supply.
    pub async fn get_oe(&self) -> bool {
        self.mutable_data.lock().await.enabled
    }

    // ------------------------------------------------------------------------------

    /// Gets the current voltage setting of the power supply.
    pub async fn get_voltage(&self) -> String {
        self.mutable_data.lock().await.voltage.clone()
    }

    // ------------------------------------------------------------------------------

    /// Gets the current current setting of the power supply.
    pub async fn get_current(&self) -> String {
        self.mutable_data.lock().await.current.clone()
    }

    // ------------------------------------------------------------------------------

    /// Enables the power supply output by sending an ON command.
    pub async fn enable_output(&self) -> anyhow::Result<PzaId> {
        trace!("[{}] Enabling output", self.psu_name);
        let payload = PowerStatePayload::from_state(PowerState::On);
        self.mqtt_client
            .pubsh(&self.topics.state_cmd, payload.to_json_bytes()?)
            .await?;
        Ok(payload.pza_id)
    }

    // ------------------------------------------------------------------------------

    /// Enables the power supply output and waits for confirmation.
    ///
    /// Returns an error if the command fails or times out.
    pub async fn enable_output_wait_ack(&self, timeout_duration: Duration) -> anyhow::Result<()> {
        // Send the enable command
        let id = self.enable_output().await?;

        // Wait for confirmation of state change
        let mut state_rx = self.subscribe_state_changes();
        let result = tokio::time::timeout(timeout_duration, async {
            loop {
                match state_rx.recv().await {
                    Ok(state_payload) => {
                        if state_payload.pza_id == id {
                            if state_payload.state == PowerState::On {
                                return Ok(());
                            } else {
                                return Err(anyhow::anyhow!(
                                    "Enable output command failed - received state: {:?}",
                                    state_payload.state
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "[{}] Error receiving output enable state: {}",
                            self.psu_name, e
                        );
                    }
                }
            }
        })
        .await;

        // Return based on the result
        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!(
                "Timeout waiting for output enable confirmation"
            )),
        }
    }

    // ------------------------------------------------------------------------------

    /// Disables the power supply output by sending an OFF command.
    pub async fn disable_output(&self) -> anyhow::Result<PzaId> {
        trace!("[{}] Disabling output", self.psu_name);
        let payload = PowerStatePayload::from_state(PowerState::Off);
        self.mqtt_client
            .pubsh(&self.topics.state_cmd, payload.to_json_bytes()?)
            .await?;
        Ok(payload.pza_id)
    }

    // ------------------------------------------------------------------------------

    /// Disables the power supply output and waits for confirmation.
    ///
    /// Returns an error if the command fails or times out.
    pub async fn disable_output_wait_ack(&self, timeout_duration: Duration) -> anyhow::Result<()> {
        // Send the disable command
        let id = self.disable_output().await?;

        // Wait for confirmation of state change
        let mut state_rx = self.subscribe_state_changes();
        let result = tokio::time::timeout(timeout_duration, async {
            loop {
                match state_rx.recv().await {
                    Ok(state_payload) => {
                        if state_payload.pza_id == id {
                            if state_payload.state == PowerState::Off {
                                return Ok(());
                            } else {
                                return Err(anyhow::anyhow!(
                                    "Disable output command failed - received state: {:?}",
                                    state_payload.state
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "[{}] Error receiving output disable state: {}",
                            self.psu_name, e
                        );
                    }
                }
            }
        })
        .await;

        // Return based on the result
        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!(
                "Timeout waiting for output disable confirmation"
            )),
        }
    }

    // ------------------------------------------------------------------------------

    /// Sets the voltage of the power supply to the specified value.
    pub async fn set_voltage(&self, voltage: String) -> anyhow::Result<PzaId> {
        trace!("[{}] Setting voltage to {}", self.psu_name, voltage);
        let payload = VoltagePayload::from_string(voltage);
        self.mqtt_client
            .pubsh(&self.topics.voltage_cmd, payload.to_json_bytes()?)
            .await?;
        Ok(payload.pza_id)
    }

    // ------------------------------------------------------------------------------

    /// Sets the voltage of the power supply and waits for confirmation.
    ///
    /// Returns an error if the command fails or times out.
    pub async fn set_voltage_wait_ack(
        &self,
        voltage: String,
        timeout_duration: Duration,
    ) -> anyhow::Result<()> {
        // Send the voltage command
        let id = self.set_voltage(voltage.clone()).await?;

        // Wait for confirmation of voltage change
        let mut voltage_rx = self.subscribe_voltage_changes();
        let result = tokio::time::timeout(timeout_duration, async {
            loop {
                match voltage_rx.recv().await {
                    Ok(voltage_payload) => {
                        if voltage_payload.pza_id == id {
                            if voltage_payload.voltage == voltage {
                                return Ok(());
                            } else {
                                return Err(anyhow::anyhow!(
                                    "Set voltage command failed - expected: {}, received: {}",
                                    voltage,
                                    voltage_payload.voltage
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        error!("[{}] Error receiving voltage update: {}", self.psu_name, e);
                    }
                }
            }
        })
        .await;

        // Return based on the result
        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!("Timeout waiting for voltage confirmation")),
        }
    }

    // ------------------------------------------------------------------------------

    /// Sets the current limit of the power supply to the specified value.
    pub async fn set_current(&self, current: String) -> anyhow::Result<PzaId> {
        trace!("[{}] Setting current to {}", self.psu_name, current);
        let payload = CurrentPayload::from_string(current);
        self.mqtt_client
            .pubsh(&self.topics.current_cmd, payload.to_json_bytes()?)
            .await?;
        Ok(payload.pza_id)
    }

    // ------------------------------------------------------------------------------

    /// Sets the current limit of the power supply and waits for confirmation.
    ///
    /// Returns an error if the command fails or times out.
    pub async fn set_current_wait_ack(
        &self,
        current: String,
        timeout_duration: Duration,
    ) -> anyhow::Result<()> {
        // Send the current command
        let id = self.set_current(current.clone()).await?;

        // Wait for confirmation of current change
        let mut current_rx = self.subscribe_current_changes();
        let result = tokio::time::timeout(timeout_duration, async {
            loop {
                match current_rx.recv().await {
                    Ok(current_payload) => {
                        if current_payload.pza_id == id {
                            if current_payload.current == current {
                                return Ok(());
                            } else {
                                return Err(anyhow::anyhow!(
                                    "Set current command failed - expected: {}, received: {}",
                                    current,
                                    current_payload.current
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        error!("[{}] Error receiving current update: {}", self.psu_name, e);
                    }
                }
            }
        })
        .await;

        // Return based on the result
        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(anyhow::anyhow!("Timeout waiting for current confirmation")),
        }
    }

    // ------------------------------------------------------------------------------
    // Dynamic Callback Management
    // ------------------------------------------------------------------------------

    /// Subscribes to output enable state changes.
    ///
    /// Returns a receiver that will receive notifications when the power
    /// supply output state changes between ON and OFF.
    pub fn subscribe_state_changes(&self) -> broadcast::Receiver<Arc<PowerStatePayload>> {
        self.state_channel.0.subscribe()
    }

    // ------------------------------------------------------------------------------

    /// Subscribes to output voltage state changes.
    ///
    /// Returns a receiver that will receive notifications when the power
    /// supply voltage setting is modified.
    pub fn subscribe_voltage_changes(&self) -> broadcast::Receiver<Arc<VoltagePayload>> {
        self.voltage_channel.0.subscribe()
    }

    // ------------------------------------------------------------------------------

    /// Subscribes to output current state changes.
    ///
    /// Returns a receiver that will receive notifications when the power
    /// supply current limit is modified.
    pub fn subscribe_current_changes(&self) -> broadcast::Receiver<Arc<CurrentPayload>> {
        self.current_channel.0.subscribe()
    }

    // ------------------------------------------------------------------------------
}
