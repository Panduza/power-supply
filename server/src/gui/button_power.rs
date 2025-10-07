use dioxus::prelude::*;
use panduza_power_supply_client::PowerSupplyClient;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    // Toggle output enable/disable
    let toggle_output = move || {
        if let Some(client_arc) = props.psu_client.clone() {
            let enabled = props.output_enabled;
            let on_output_changed = props.on_output_changed.clone();
            let on_status_message = props.on_status_message.clone();

            spawn(async move {
                let client = client_arc.lock().await;
                let result = if enabled {
                    client.disable_output().await
                } else {
                    client.enable_output().await
                };

                match result {
                    Ok(()) => {
                        on_output_changed.call(!enabled);
                        on_status_message.call("Output toggled successfully".to_string());
                    }
                    Err(e) => {
                        on_status_message.call(format!("Error toggling output: {}", e));
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "power-button-container glass-card",

            div {
                class: "component-header",
                div {
                    class: if props.output_enabled {
                        "component-icon btn-success"
                    } else {
                        "component-icon btn-error"
                    },
                    if props.output_enabled { "⚡" } else { "⏸" }
                }
                h3 {
                    class: "component-title",
                    "Power Output"
                }
            }

            button {
                class: if props.output_enabled {
                    "btn btn-error power-button"
                } else {
                    "btn btn-success power-button"
                },
                onclick: move |_| toggle_output(),
                if props.output_enabled { "Turn OFF" } else { "Turn ON" }
            }
        }
    }
}
