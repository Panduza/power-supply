use crate::client::PowerSupplyClient;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Props, Clone)]
pub struct CurrentSetterProps {
    /// Current current value
    pub current: String,
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
    /// Callback when the current value changes
    pub on_current_changed: EventHandler<String>,
    /// Callback when there's a status message to display
    pub on_status_message: EventHandler<String>,
}

impl PartialEq for CurrentSetterProps {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current && self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn CurrentSetter(props: CurrentSetterProps) -> Element {
    let mut local_current = use_signal(|| props.current.clone());

    // Update local current when props change
    use_effect(move || {
        local_current.set(props.current.clone());
    });

    // Set current function
    let set_current = move || {
        if let Some(client_arc) = props.psu_client.clone() {
            let curr = local_current();
            let on_status_message = props.on_status_message.clone();

            spawn(async move {
                let client = client_arc.lock().await;
                match client.set_current(curr.clone()).await {
                    Ok(()) => {
                        on_status_message.call(format!("Current limit set to {} A", curr));
                    }
                    Err(e) => {
                        on_status_message.call(format!("Error setting current: {}", e));
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "current-setter-container",

            div {
                class: "component-header",
                span {
                    class: "current-setter-icon",
                    "🔋"
                }
                span {
                    class: "current-setter-label",
                    "Current Limit"
                }
            }

            div {
                class: "input-group",
                input {
                    class: "form-input",
                    r#type: "number",
                    step: "0.01",
                    min: "0",
                    placeholder: "0.00",
                    value: local_current(),
                    oninput: move |evt| {
                        local_current.set(evt.value());
                        props.on_current_changed.call(evt.value());
                    }
                }
                span {
                    class: "px-3 py-2 bg-gray-100 border border-gray-300 rounded-r-md text-sm font-medium text-gray-700",
                    "A"
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| set_current(),
                    "Set"
                }
            }
        }
    }
}
