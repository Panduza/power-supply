use crate::server::state::ServerState;
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(Props, Clone)]
pub struct DeviceSelectorProps {
    /// Currently selected device name
    pub selected_device: String,
    /// List of available device names
    pub instances_names: Vec<String>,
    /// Callback when the device selection changes
    pub on_device_changed: EventHandler<String>,
}

impl PartialEq for DeviceSelectorProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_device == other.selected_device
            && self.instances_names == other.instances_names
    }
}

#[component]
pub fn DeviceSelector(props: DeviceSelectorProps) -> Element {
    let server_state = use_context::<Arc<ServerState>>();

    info!("Rendering DeviceSelector {:?}", server_state);

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
                value: props.selected_device.clone(),
                onchange: move |evt| {
                    props.on_device_changed.call(evt.value());
                },
                option { value: "", "Select a device..." }
                if let Some(names) = instance_names.read().as_ref() {
                    for name in names.iter() {
                        option { value: name.clone(), "{name}" }
                    }
                }
            }
        }
    }
}
