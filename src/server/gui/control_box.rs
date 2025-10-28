use crate::client::PowerSupplyClient;

use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

mod button_power;
mod current_setter;
mod instance_selector;
mod voltage_setter;

pub use button_power::PowerButton;
pub use instance_selector::InstanceSelector;
pub use voltage_setter::VoltageSetter;

#[derive(Props, Clone)]
pub struct ControlBoxProps {
    /// The instance client for controlling the power supply
    pub instance_client: Option<Arc<Mutex<PowerSupplyClient>>>,

    /// Currently selected instance name
    pub selected_instance: Option<String>,
    /// List of available instance names
    pub instances_names: Option<Vec<String>>,
    /// Callback when the instance selection changes
    pub on_instance_changed: EventHandler<String>,
}

impl PartialEq for ControlBoxProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_instance == other.selected_instance
            && self.instances_names == other.instances_names
    }
}

#[component]
pub fn ControlBox(props: ControlBoxProps) -> Element {
    // Check if no instance is available
    let no_instance_available =
        props.instances_names.is_none() || props.instances_names.as_ref().unwrap().is_empty();

    // Check if not initialized
    let not_initialized = props.instance_client.is_none()
        || props.selected_instance.is_none()
        || no_instance_available;

    if no_instance_available {
        return rsx! {
            div {
                class: "control-box-container",
                span { "No instance available. Please check the server configuration." }
            }
        };
    } else if not_initialized {
        return rsx! {
            div {
                class: "control-box-container",
                span { "Control box not initialized. Please check the server configuration." }
            }
        };
    } else {
        let i_client = props.instance_client.clone().expect("no instance client");
        let selection = props
            .selected_instance
            .clone()
            .expect("no selected instance");
        let names = props.instances_names.clone().expect("no instances names");
        let on_instance_changed = props.on_instance_changed.clone();

        // Rendering the button
        rsx! {
            div {
                class: "control-box-container",

                InstanceSelector {
                    selected_instance: selection,
                    instances_names: names,
                    on_instance_changed: move |selected_instance| {
                        on_instance_changed.call(selected_instance);
                    },
                }

                PowerButton {
                    instance_client: i_client.clone(),
                }

                VoltageSetter {
                    instance_client: i_client.clone(),
                }
            }
        }
    }
}
