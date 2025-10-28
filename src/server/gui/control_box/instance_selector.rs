use crate::server::state::ServerState;
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(Props, Clone)]
pub struct InstanceSelectorProps {
    /// Currently selected instance name
    pub selected_instance: String,
    /// List of available instance names
    pub instances_names: Vec<String>,
    /// Callback when the instance selection changes
    pub on_instance_changed: EventHandler<String>,
}

impl PartialEq for InstanceSelectorProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_instance == other.selected_instance
            && self.instances_names == other.instances_names
    }
}

#[component]
pub fn InstanceSelector(props: InstanceSelectorProps) -> Element {
    rsx! {
        div {
            class: "device-selector-container",

            select {
                class: "form-select",
                value: props.selected_instance.clone(),
                onchange: move |evt| {
                    props.on_instance_changed.call(evt.value());
                },
                // option { value: "", "Select an instance..." }

                for name in props.instances_names.iter() {
                    option { value: name.clone(), "{name}" }
                }

            }
        }
    }
}
