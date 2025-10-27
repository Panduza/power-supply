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
    let server_state = use_context::<Arc<ServerState>>();

    // info!("Rendering InstanceSelector {:?}", server_state);

    let mut instance_names: Signal<Option<Vec<String>>> = use_signal(|| None);

    {
        let server_state = server_state.clone();
        use_effect(move || {
            let server_state = server_state.clone();
            spawn(async move {
                let instances = server_state.instances.lock().await;
                let names: Vec<String> = instances.keys().cloned().collect();
                instance_names.set(Some(names));
            });
        });
    }

    rsx! {
        div {
            class: "device-selector-container",

            select {
                class: "form-select",
                value: props.selected_instance.clone(),
                onchange: move |evt| {
                    props.on_instance_changed.call(evt.value());
                },
                option { value: "", "Select an instance..." }
                if let Some(names) = instance_names.read().as_ref() {
                    for name in names.iter() {
                        option { value: name.clone(), "{name}" }
                    }
                }
            }
        }
    }
}
