use dioxus::prelude::*;

#[derive(Props, Clone)]
pub struct DeviceSelectorProps {
    /// Currently selected device name
    pub selected_device: String,
    /// List of available device names
    pub device_names: Vec<String>,
    /// Callback when the device selection changes
    pub on_device_changed: EventHandler<String>,
}

impl PartialEq for DeviceSelectorProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_device == other.selected_device && self.device_names == other.device_names
    }
}

#[component]
pub fn DeviceSelector(props: DeviceSelectorProps) -> Element {
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
                for name in props.device_names.iter() {
                    option { value: name.clone(), "{name}" }
                }
            }
        }
    }
}
