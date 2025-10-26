use crate::client::PowerSupplyClient;
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

    // Effect to setup callback when component mounts or client changes
    use_effect({
        let psu_client = props.psu_client.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                spawn(async move {
                    // Get initial output enable state
                    let initial_oe = client_arc.lock().await.get_oe().await;
                    output_state.set(Some(initial_oe));

                    // Add new callback to listen for OE changes from MQTT
                    let mut oe_changes = client_arc.lock().await.subscribe_oe_changes();

                    // Listen for messages from MQTT callback and update UI state
                    spawn(async move {
                        loop {
                            let notification = oe_changes.recv().await;

                            match notification {
                                Ok(enabled) => output_state.set(Some(enabled)),
                                Err(_) => todo!(),
                            }
                        }
                    });
                });
            }
        }
    });

    // Toggle output enable/disable
    let mut toggle_output = {
        let psu_client = props.psu_client.clone();

        move || {
            if let Some(client_arc) = psu_client.clone() {
                let enabled = output_state.read().clone().unwrap_or(false);

                // Set state to undefined immediately when user clicks
                output_state.set(None);

                spawn(async move {
                    let client = client_arc.lock().await;
                    let result = if enabled {
                        client.disable_output().await
                    } else {
                        client.enable_output().await
                    };
                });
            }
        }
    };

    // Rendering the button
    rsx! {
        div {
            class: "power-button-container",

            // Status display
            div {
                class: match *output_state.read() {
                    Some(true) => "power-button-status-on",
                    Some(false) => "power-button-status-off",
                    None => "power-button-status-unknown",
                },
                match *output_state.read() {
                    Some(true) => "POWER ENABLED",
                    Some(false) => "POWER DISABLED",
                    None => "UPDATING...",
                }
            }

            // Toggle button
            button {
                class: match *output_state.read() {
                    Some(true) => "power-button-toggle-enabled",
                    Some(false) => "power-button-toggle-enabled",
                    None => "power-button-toggle-disabled",
                },
                onclick: move |_| match *output_state.read() {
                    Some(true) =>  toggle_output(),
                    Some(false) =>  toggle_output(),
                    None => {},
                },
            }
        }
    }
}
