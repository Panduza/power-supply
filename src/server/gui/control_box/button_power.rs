use pza_power_supply_client::PowerSupplyClient;
/// Power Button
///
/// Requirements
/// - The power button must be able to toggle the output state
/// - If the power is enabled display "ON" and color must be Green
/// - If the power is disabled display "OFF" and color must be Red
///
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, trace};

#[derive(Props, Clone)]
pub struct PowerButtonProps {
    /// The instance client for controlling the power supply
    pub instance_client: Arc<Mutex<PowerSupplyClient>>,
    /// Keyboard shortcut key for power toggle
    pub toggle_key: Option<String>,
}

impl PartialEq for PowerButtonProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.instance_client, &other.instance_client)
            && self.toggle_key == other.toggle_key
    }
}

#[component]
pub fn PowerButton(props: PowerButtonProps) -> Element {
    // Signal to track the current output state locally
    // 3 state possible: Some(true), Some(false), None (unknown)
    let mut output_state: Signal<Option<bool>> = use_signal(|| None);

    // Setup MQTT subscription for output state changes using effect
    use_effect({
        let instance_client = props.instance_client.clone();
        move || {
            let instance_client = instance_client.clone();

            trace!("EFFECT BPOWER");

            // Reset state when client changes
            output_state.set(None);

            spawn(async move {
                trace!("Setting up output state subscription for new client");

                // Get initial output enable state
                let initial_oe = instance_client.lock().await.get_oe().await;
                output_state.set(Some(initial_oe));

                // Add new callback to listen for OE changes from MQTT
                let mut oe_changes = instance_client.lock().await.subscribe_oe_changes();

                // Listen for messages from MQTT callback and update UI state
                loop {
                    let notification = oe_changes.recv().await;

                    match notification {
                        Ok(enabled) => output_state.set(Some(enabled)),
                        Err(_) => break, // Exit loop on error
                    }
                }
            });
        }
    });

    // Toggle output enable/disable
    let toggle_output = {
        let instance_client = props.instance_client.clone();

        move |_| {
            // Log the toggle action
            trace!("Toggling power output");

            // Read the current state once and store it
            let current_enabled = output_state.read().clone().unwrap_or(false);

            // Set state to undefined immediately when user clicks
            output_state.set(None);

            // Clone the client for the async block to avoid moving it
            let instance_client = instance_client.clone();
            spawn(async move {
                let client = instance_client.lock().await;
                trace!("Toggle output - client = {}", client.name());
                let result = if current_enabled {
                    client.disable_output().await
                } else {
                    client.enable_output().await
                };

                if let Err(e) = result {
                    info!("Error toggling power output: {:?}", e);
                    // Reset to previous state on error
                    output_state.set(Some(current_enabled));
                }
            });
        }
    };

    // Get current state for rendering (read once)
    let current_state = output_state.read().clone();

    info!("BBB RELOAD");

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // Rendering
    rsx! {
        div {
            class: "power-button-container",

            // Status display
            div {
                class: "power-button-label",
                "POWER"
            }

            // Status display
            div {
                class: match current_state {
                    Some(true) => "power-button-status on",
                    Some(false) => "power-button-status off",
                    None => "power-button-status unknown",
                },
                match current_state {
                    Some(true) => "ENABLED",
                    Some(false) => "DISABLED",
                    None => "UPDATING...",
                }
            }

            // Toggle button
            button {
                class: match current_state {
                    Some(true) => "power-button-toggle enabled",
                    Some(false) => "power-button-toggle enabled",
                    None => "power-button-toggle disabled",
                },
                disabled: current_state.is_none(),
                onclick: toggle_output,
                {
                    if let Some(key) = &props.toggle_key {
                        format!("TOGGLE ({})", key.to_uppercase())
                    } else {
                        "TOGGLE".to_string()
                    }
                }
            }
        }
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
}
