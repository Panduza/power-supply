use crate::client::PowerSupplyClient;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Props, Clone)]
pub struct VoltageSetterProps {
    /// The instance client for controlling the power supply
    pub instance_client: Arc<Mutex<PowerSupplyClient>>,
}

impl PartialEq for VoltageSetterProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.instance_client, &other.instance_client)
    }
}

#[component]
pub fn VoltageSetter(props: VoltageSetterProps) -> Element {
    let mut s_voltage_real: Signal<Option<String>> = use_signal(|| None);
    let mut s_voltage_request: Signal<Option<String>> = use_signal(|| None);

    // Setup MQTT subscription for output state changes using coroutine
    let _subscription_coroutine = use_coroutine({
        let instance_client = props.instance_client.clone();
        move |_rx: UnboundedReceiver<()>| {
            let instance_client = instance_client.clone();
            async move {
                trace!("Setting up output state subscription");

                // Get initial output enable state
                let initial_value = instance_client.lock().await.get_voltage().await;
                s_voltage_real.set(Some(initial_value));

                // Add new callback to listen for voltage changes from MQTT
                let mut changes = instance_client.lock().await.subscribe_voltage_changes();

                // Listen for messages from MQTT callback and update UI state
                loop {
                    let notification = changes.recv().await;
                    match notification {
                        Ok(voltage) => {
                            s_voltage_real.set(Some(voltage));
                            s_voltage_request.set(None);
                        }
                        Err(_) => break, // Exit loop on error
                    }
                }
            }
        }
    });

    // Set voltage function
    let set_voltage = move || {
        let instance_client = props.instance_client.clone();
        if let Some(volt) = s_voltage_request.read().clone() {
            spawn(async move {
                let client = instance_client.lock().await;
                client.set_voltage(volt.clone()).await.expect("ahaha");
            });
        }
    };

    let real = s_voltage_real.read().clone();
    let request = s_voltage_request.read().clone();

    rsx! {
        div {
            class: "voltage-setter-container",

            div {
                class: "voltage-setter-label",
                "Voltage"
            }

            div {
                class: "input-group",
                input {
                    class: "form-input",
                    r#type: "number",
                    step: "0.1",
                    min: "0",
                    placeholder: "0.0",
                    value: match request {
                        Some(v) => v,
                        None => {
                            match real {
                                Some(v) => v,
                                None => "".to_string(),
                            }
                        }
                    },
                    oninput: move |evt| {
                        s_voltage_request.set(Some(evt.value()));
                    }
                }
                span {
                    "V"
                }
            }

            button {
                class: "voltage-setter-button",
                disabled: s_voltage_request.read().is_none(),
                onclick: move |_| set_voltage(),
                "Set"
            }
        }
    }
}
