use dioxus::prelude::*;
use panduza_power_supply_client::{PowerSupplyClient, PowerSupplyClientBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

mod button_power;
mod current_setter;
mod device_selector;
mod voltage_setter;

use button_power::PowerButton;
use current_setter::CurrentSetter;
use device_selector::DeviceSelector;
use voltage_setter::VoltageSetter;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Gui() -> Element {
    let mut runtime_status = use_signal(|| "Initializing...".to_string());

    // Use effect to monitor runtime status
    use_effect(move || {
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            runtime_status.set("Background services running".to_string());
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            header {
                h1 {
                    "Panduza Power Supply"
                }
            }

            main {
                PowerSupplyControl {}
            }
        }
    }
}

#[component]
pub fn PowerSupplyControl() -> Element {
    let app_state = use_context::<crate::AppState>();
    let mut selected_psu = use_signal(|| String::new());
    let mut output_enabled = use_signal(|| false);
    let mut voltage = use_signal(|| "0.0".to_string());
    let mut current = use_signal(|| "0.0".to_string());
    let mut status_message = use_signal(|| "Ready".to_string());
    let mut psu_names = use_signal(|| Vec::<String>::new());
    let mut psu_client: Signal<Option<Arc<Mutex<PowerSupplyClient>>>> = use_signal(|| None);

    // Load PSU names from app state
    {
        let psu_names_arc = app_state.psu_names.clone();
        use_effect(move || {
            let psu_names_arc = psu_names_arc.clone();
            spawn(async move {
                loop {
                    let names = psu_names_arc.lock().await;
                    if !names.is_empty() {
                        psu_names.set(names.clone());
                        if selected_psu().is_empty() && !names.is_empty() {
                            selected_psu.set(names[0].clone());
                        }
                        break;
                    }
                    drop(names);
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            });
        });
    }

    // Create PSU client when selection changes
    {
        let broker_config_arc = app_state.broker_config.clone();
        use_effect(move || {
            let selected = selected_psu();

            if !selected.is_empty() {
                let broker_config_arc = broker_config_arc.clone();
                spawn(async move {
                    let broker_config = broker_config_arc.lock().await;
                    if let Some(config) = broker_config.as_ref() {
                        let client = PowerSupplyClientBuilder::from_broker_config(config.clone())
                            .with_power_supply_name(selected.clone())
                            .build();

                        psu_client.set(Some(Arc::new(Mutex::new(client))));
                        status_message.set(format!("Connected to {}", selected));
                    }
                });
            }
        });
    }

    // Refresh current state from PSU
    let _refresh_state = move || {
        if let Some(client_arc) = psu_client() {
            spawn(async move {
                let client = client_arc.lock().await;
                let enabled = client.get_oe().await;
                let volt = client.get_voltage().await;
                let curr = client.get_current().await;

                output_enabled.set(enabled);
                voltage.set(volt);
                current.set(curr);
            });
        }
    };

    // Callbacks for PowerButton component
    let on_output_changed = move |enabled: bool| {
        output_enabled.set(enabled);
    };

    let on_status_message = move |message: String| {
        status_message.set(message);
    };

    // Callbacks for VoltageSetter component
    let on_voltage_changed = move |new_voltage: String| {
        voltage.set(new_voltage);
    };

    // Callbacks for CurrentSetter component
    let on_current_changed = move |new_current: String| {
        current.set(new_current);
    };

    // Callbacks for DeviceSelector component
    let on_device_changed = move |new_device: String| {
        selected_psu.set(new_device);
    };

    rsx! {
        div {
            class: "max-w-4xl mx-auto space-y-6",

            // Status Card
            div {
                class: "text-center mb-6",
                div {
                    class: {
                        if status_message().contains("Error") {
                            "status-message error"
                        } else if status_message().contains("Connected") || status_message().contains("successfully") {
                            "status-message success"
                        } else {
                            "status-message info"
                        }
                    },
                    "{status_message}"
                }
            }

            // PSU Selection Card - Using DeviceSelector component
            DeviceSelector {
                selected_device: selected_psu(),
                device_names: psu_names(),
                on_device_changed: on_device_changed,
            }

            if psu_names().is_empty() {
                // No PSUs available message
                div {
                    class: "glass-card text-center",

                    h3 {
                        class: "component-title text-2xl mb-3",
                        "No Devices Found"
                    }
                    p {
                        class: "text-slate-500 text-lg mb-6",
                        "No power supply devices are configured or detected."
                    }
                    div {
                        class: "status-message info",
                        span { "💡 Check your configuration file" }
                    }
                }
            } else if !selected_psu().is_empty() {
                // Control Panel
                div {
                    class: "control-grid",

                    // Output Control Card - Using PowerButton component
                    div {
                        PowerButton {
                            output_enabled: output_enabled(),
                            psu_client: psu_client(),
                            on_output_changed: on_output_changed,
                            on_status_message: on_status_message,
                        }
                    }

                    // Voltage and Current Control Card
                    div {
                        class: "glass-card",

                        div {
                            class: "space-y-6",

                            // Voltage Control Component
                            VoltageSetter {
                                voltage: voltage(),
                                psu_client: psu_client(),
                                on_voltage_changed: on_voltage_changed,
                                on_status_message: on_status_message,
                            }

                            // Current Control Component
                            CurrentSetter {
                                current: current(),
                                psu_client: psu_client(),
                                on_current_changed: on_current_changed,
                                on_status_message: on_status_message,
                            }
                        }
                    }
                }
            }
        }
    }
}
