use crate::client::PowerSupplyClient;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Props, Clone)]
pub struct CurrentSetterProps {
    /// The instance client for controlling the power supply
    pub instance_client: Arc<Mutex<PowerSupplyClient>>,
}

impl PartialEq for CurrentSetterProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.instance_client, &other.instance_client)
    }
}

#[component]
pub fn CurrentSetter(props: CurrentSetterProps) -> Element {
    let mut s_current_real: Signal<Option<String>> = use_signal(|| None);
    let mut s_current_request: Signal<Option<String>> = use_signal(|| None);

    // Setup MQTT subscription for current changes using coroutine
    let _subscription_coroutine = use_coroutine({
        let instance_client = props.instance_client.clone();
        move |_rx: UnboundedReceiver<()>| {
            let instance_client = instance_client.clone();
            async move {
                trace!("Setting up current subscription");

                // Get initial current value
                let initial_value = instance_client.lock().await.get_current().await;
                s_current_real.set(Some(initial_value));

                // Add new callback to listen for current changes from MQTT
                let mut changes = instance_client.lock().await.subscribe_current_changes();

                // Listen for messages from MQTT callback and update UI state
                loop {
                    let notification = changes.recv().await;
                    match notification {
                        Ok(current) => {
                            s_current_real.set(Some(current));
                            s_current_request.set(None);
                        }
                        Err(_) => break, // Exit loop on error
                    }
                }
            }
        }
    });

    // Set current function
    let set_current = move || {
        let instance_client = props.instance_client.clone();
        if let Some(curr) = s_current_request.read().clone() {
            spawn(async move {
                let client = instance_client.lock().await;
                client.set_current(curr.clone()).await.expect("ahaha");
            });
        }
    };

    let real = s_current_real.read().clone();
    let request = s_current_request.read().clone();

    rsx! {
        div {
            class: "current-setter-container",

            div {
                class: "current-setter-label",
                "Current Limit"
            }

            div {
                class: "input-group",
                input {
                    class: "form-input",
                    r#type: "number",
                    step: "0.01",
                    min: "0",
                    placeholder: "0.00",
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
                        s_current_request.set(Some(evt.value()));
                    }
                }
                span {
                    "A"
                }
            }

            button {
                class: "current-setter-button",
                disabled: s_current_request.read().is_none(),
                onclick: move |_| set_current(),
                "Set"
            }
        }
    }
}
