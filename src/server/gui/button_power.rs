use crate::client::client::CallbackId;
use crate::client::PowerSupplyClient;
/// Power Button
///
/// Requirements
/// - The power button must be able to toggle the output state
/// - If the power is enabled display "ON" and color must be Green
/// - If the power is disabled display "OFF" and color must be Red
///
use dioxus::prelude::*;
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::info;

#[derive(Props, Clone)]
pub struct PowerButtonProps {
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
}

impl PartialEq for PowerButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn PowerButton(props: PowerButtonProps) -> Element {
    // Signal to track the current output state locally
    // 3 state possible: Some(true), Some(false), None (unknown)
    let mut output_state: Signal<Option<bool>> = use_signal(|| None);

    // Signal to store the callback ID for cleanup
    let mut callback_id = use_signal(|| None::<CallbackId>);

    // Effect to setup callback when component mounts or client changes
    use_effect({
        let psu_client = props.psu_client.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                spawn(async move {
                    // Get initial output enable state
                    let initial_oe = client_arc.lock().await.get_oe().await;
                    output_state.set(Some(initial_oe));

                    // Create a channel for communication between MQTT callback and UI
                    let (tx, mut rx) = mpsc::unbounded_channel::<bool>();

                    // Create the callback function
                    let mqtt_callback = {
                        let tx = tx.clone();
                        move |enabled: bool| -> BoxFuture<'static, ()> {
                            let tx = tx.clone();
                            Box::pin(async move {
                                info!("MQTT callback triggered with value: {}", enabled);
                                let _ = tx.send(enabled);
                            })
                        }
                    };

                    // Add new callback to listen for OE changes from MQTT
                    let new_callback_id = client_arc
                        .lock()
                        .await
                        .oe_callbacks()
                        .lock()
                        .await
                        .add(Box::new(mqtt_callback));

                    // Store the callback ID for later cleanup
                    callback_id.set(Some(new_callback_id));

                    // Spawn a task to listen for messages from the MQTT callback
                    spawn(async move {
                        while let Some(enabled) = rx.recv().await {
                            // Update local state
                            output_state.set(Some(enabled));
                        }
                    });
                });
            }
        }
    });

    // Toggle output enable/disable
    let toggle_output = {
        let psu_client = props.psu_client.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                let enabled = output_state.read().clone().unwrap_or(false);

                spawn(async move {
                    let client = client_arc.lock().await;
                    let result = if enabled {
                        client.disable_output().await
                    } else {
                        client.enable_output().await
                    };

                    match result {
                        Ok(()) => {
                            // Update local state immediately for responsive UI
                            output_state.set(Some(!enabled));
                        }
                        Err(e) => {} // Handle error (e.g., show notification)
                    }
                });
            }
        }
    };

    // Rendering the button
    rsx! {
        div {
            class: "power-button-container",
            button {
                class: match *output_state.read() {
                    Some(true) => "power-button-on",  // Green when ON
                    Some(false) => "power-button-off", // Red when OFF
                    None => "power-button-unknown",   // Wait animation when unknown
                },
                onclick: move |_| toggle_output(),
                match *output_state.read() {
                    Some(true) => "POWER ENABLED",
                    Some(false) => "POWER DISABLED",
                    None => "UNKNOWN",
                }
            }
        }
    }
}
