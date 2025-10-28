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

#[derive(Props, Clone)]
pub struct ControlBoxProps {
    /// The instance client for controlling the power supply
    pub instance_client: Arc<Mutex<PowerSupplyClient>>,

    /// Currently selected instance name
    pub selected_instance: String,
    /// List of available instance names
    pub instances_names: Vec<String>,
    /// Callback when the instance selection changes
    pub on_instance_changed: EventHandler<String>,
}

impl PartialEq for ControlBoxProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.instance_client, &other.instance_client)
            && self.selected_instance == other.selected_instance
            && self.instances_names == other.instances_names
    }
}

#[component]
pub fn ControlBox(props: ControlBoxProps) -> Element {
    let on_instance_changed = props.on_instance_changed.clone();

    // Rendering the button
    rsx! {
        div {
            class: "control-box-container",

            InstanceSelector {
                selected_instance: props.selected_instance.clone(),
                instances_names: props.instances_names.clone(),
                on_instance_changed: move |selected_instance| {
                    on_instance_changed.call(selected_instance);
                },
            }

            PowerButton {
                instance_client: props.instance_client.clone(),
            }
        }
    }
}
