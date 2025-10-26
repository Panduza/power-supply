use crate::client::PowerSupplyClient;

use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

mod button_power;
mod current_setter;
mod device_selector;
mod voltage_setter;

pub use button_power::PowerButton;
pub use device_selector::DeviceSelector;

#[derive(Props, Clone)]
pub struct ControlBoxProps {
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
}

impl PartialEq for ControlBoxProps {
    fn eq(&self, other: &Self) -> bool {
        self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn ControlBox(props: ControlBoxProps) -> Element {
    // Rendering the button
    rsx! {
        div {

            DeviceSelector {
                selected_device: "".to_string(),
                device_names: vec![],
                on_device_changed: |_| {},
            }

            PowerButton {
                psu_client: props.psu_client.clone(),
            }
        }
    }
}
