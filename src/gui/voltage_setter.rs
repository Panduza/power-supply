use crate::client::PowerSupplyClient;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Props, Clone)]
pub struct VoltageSetterProps {
    /// Current voltage value
    pub voltage: String,
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
    /// Callback when the voltage value changes
    pub on_voltage_changed: EventHandler<String>,
    /// Callback when there's a status message to display
    pub on_status_message: EventHandler<String>,
}

impl PartialEq for VoltageSetterProps {
    fn eq(&self, other: &Self) -> bool {
        self.voltage == other.voltage && self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn VoltageSetter(props: VoltageSetterProps) -> Element {
    let mut local_voltage = use_signal(|| props.voltage.clone());

    // Update local voltage when props change
    use_effect(move || {
        local_voltage.set(props.voltage.clone());
    });

    // Set voltage function
    let set_voltage = move || {
        if let Some(client_arc) = props.psu_client.clone() {
            let volt = local_voltage();
            let on_status_message = props.on_status_message.clone();

            spawn(async move {
                let client = client_arc.lock().await;
                match client.set_voltage(volt.clone()).await {
                    Ok(()) => {
                        on_status_message.call(format!("Voltage set to {} V", volt));
                    }
                    Err(e) => {
                        on_status_message.call(format!("Error setting voltage: {}", e));
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "voltage-setter-container",

            div {
                class: "component-header",
                span {
                    class: "voltage-setter-icon",
                    "⚡"
                }
                span {
                    class: "voltage-setter-label",
                    "Voltage Control"
                }
            }

            div {
                class: "input-group",
                input {
                    class: "form-input",
                    r#type: "number",
                    step: "0.1",
                    min: "0",
                    placeholder: "0.0",
                    value: local_voltage(),
                    oninput: move |evt| {
                        local_voltage.set(evt.value());
                        props.on_voltage_changed.call(evt.value());
                    }
                }
                span {
                    class: "px-3 py-2 bg-gray-100 border border-gray-300 rounded-r-md text-sm font-medium text-gray-700",
                    "V"
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| set_voltage(),
                    "Set"
                }
            }
        }
    }
}
