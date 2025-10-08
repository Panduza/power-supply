/// Power Button
///
/// Requirements
/// - The power button must be able to toggle the output state
/// - If the power is enabled display ON and color must be Green
/// - If the power is disabled display OFF and color must be Red
///
use dioxus::prelude::*;
use panduza_power_supply_client::client::CallbackId;
use panduza_power_supply_client::PowerSupplyClient;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Props, Clone)]
pub struct PowerButtonProps {
    /// Whether the output is currently enabled
    pub output_enabled: bool,
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
    /// Callback when the output state changes
    pub on_output_changed: EventHandler<bool>,
    /// Callback when there's a status message to display
    pub on_status_message: EventHandler<String>,
}

impl PartialEq for PowerButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.output_enabled == other.output_enabled
            && self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn PowerButton(props: PowerButtonProps) -> Element {
    // Signal to track the current output state locally
    let mut output_state = use_signal(|| props.output_enabled);

    // Signal to store the callback ID for cleanup
    let mut callback_id = use_signal(|| None::<CallbackId>);

    // Effect to setup callback when component mounts or client changes
    use_effect({
        let psu_client = props.psu_client.clone();
        let on_output_changed = props.on_output_changed.clone();
        let on_status_message = props.on_status_message.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                spawn(async move {
                    // Remove previous callback if exists
                    if let Some(id) = callback_id.read().clone() {
                        client_arc.lock().await.remove_oe_callback(id).await;
                    }

                    // Create a channel for communication between MQTT callback and UI
                    let (tx, mut rx) = mpsc::unbounded_channel::<bool>();

                    // Add new callback to listen for OE changes from MQTT
                    let new_callback_id = client_arc
                        .lock()
                        .await
                        .add_oe_callback(move |enabled| {
                            let tx = tx.clone();
                            Box::pin(async move {
                                // Send the state change through the channel
                                let _ = tx.send(enabled);
                            })
                        })
                        .await;

                    // Store the callback ID for later cleanup
                    callback_id.set(Some(new_callback_id));

                    // Spawn a task to listen for messages from the MQTT callback
                    spawn(async move {
                        while let Some(enabled) = rx.recv().await {
                            // Update local state
                            output_state.set(enabled);
                            // Notify parent component
                            on_output_changed.call(enabled);
                            // Show status message
                            let status = if enabled {
                                "Output enabled via MQTT"
                            } else {
                                "Output disabled via MQTT"
                            };
                            on_status_message.call(status.to_string());
                        }
                    });
                });
            }
        }
    });

    // Cleanup effect when component unmounts
    use_drop({
        let psu_client = props.psu_client.clone();
        move || {
            if let Some(client_arc) = psu_client.clone() {
                if let Some(id) = callback_id.read().clone() {
                    spawn(async move {
                        client_arc.lock().await.remove_oe_callback(id).await;
                    });
                }
            }
        }
    });

    // Toggle output enable/disable
    let toggle_output = {
        let psu_client = props.psu_client.clone();
        let on_output_changed = props.on_output_changed.clone();
        let on_status_message = props.on_status_message.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                let enabled = output_state.read().clone();

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
                            output_state.set(!enabled);
                            on_output_changed.call(!enabled);
                            on_status_message.call("Output toggle command sent".to_string());
                        }
                        Err(e) => {
                            on_status_message.call(format!("Error toggling output: {}", e));
                        }
                    }
                });
            }
        }
    };

    rsx! {
        div {
            class: "power-button-container glass-card",

            div {
                class: "component-header",
                div {
                    class: if *output_state.read() {
                        "component-icon btn-success"
                    } else {
                        "component-icon btn-error"
                    },
                    if *output_state.read() { "⚡" } else { "⏸" }
                }
                h3 {
                    class: "component-title",
                    "Power Output"
                }
            }

            button {
                class: if *output_state.read() {
                    "btn btn-error power-button"
                } else {
                    "btn btn-success power-button"
                },
                onclick: move |_| toggle_output(),
                if *output_state.read() { "Turn OFF" } else { "Turn ON" }
            }
        }
    }
}
