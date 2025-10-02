use dioxus::prelude::*;
use panduza_power_supply_client::{PowerSupplyClient, PowerSupplyClientBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "runtime-status",
            style: "position: fixed; top: 10px; right: 10px; background: #333; color: white; padding: 10px; border-radius: 5px; font-size: 12px;",
            "Status: {runtime_status}"
        }

        div {
            class: "container mx-auto p-4",
            PowerSupplyControl {}
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
    let refresh_state = move || {
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

    // Toggle output enable/disable
    let toggle_output = move || {
        if let Some(client_arc) = psu_client() {
            let enabled = output_enabled();
            spawn(async move {
                let client = client_arc.lock().await;
                let result = if enabled {
                    client.disable_output().await
                } else {
                    client.enable_output().await
                };

                match result {
                    Ok(()) => {
                        output_enabled.set(!enabled);
                        status_message.set("Output toggled successfully".to_string());
                    }
                    Err(e) => {
                        status_message.set(format!("Error toggling output: {}", e));
                    }
                }
            });
        }
    };

    // Set voltage
    let set_voltage_fn = move || {
        if let Some(client_arc) = psu_client() {
            let volt = voltage();
            spawn(async move {
                let client = client_arc.lock().await;
                match client.set_voltage(volt.clone()).await {
                    Ok(()) => {
                        status_message.set(format!("Voltage set to {}", volt));
                    }
                    Err(e) => {
                        status_message.set(format!("Error setting voltage: {}", e));
                    }
                }
            });
        }
    };

    // Set current
    let set_current_fn = move || {
        if let Some(client_arc) = psu_client() {
            let curr = current();
            spawn(async move {
                let client = client_arc.lock().await;
                match client.set_current(curr.clone()).await {
                    Ok(()) => {
                        status_message.set(format!("Current limit set to {}", curr));
                    }
                    Err(e) => {
                        status_message.set(format!("Error setting current: {}", e));
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "bg-white shadow-lg rounded-lg p-6 mt-8 max-w-2xl mx-auto",
            h2 {
                class: "text-2xl font-bold text-gray-800 mb-6",
                "Power Supply Control"
            }

            // Status message
            div {
                class: "mb-4 p-3 bg-blue-50 border border-blue-200 rounded",
                "Status: {status_message}"
            }

            // PSU Selection
            div {
                class: "mb-6",
                label {
                    class: "block text-sm font-medium text-gray-700 mb-2",
                    "Select Power Supply:"
                }
                select {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                    value: selected_psu(),
                    onchange: move |evt| {
                        selected_psu.set(evt.value());
                    },
                    option { value: "", "Select a PSU..." }
                    for name in psu_names() {
                        option { value: name.clone(), "{name}" }
                    }
                }
            }

            if psu_names().is_empty() {
                // No PSUs available message
                div {
                    class: "text-center py-12",
                    div {
                        class: "text-6xl mb-4",
                        "⚡"
                    }
                    h3 {
                        class: "text-xl font-semibold text-gray-600 mb-2",
                        "No Power Supplies Available"
                    }
                    p {
                        class: "text-gray-500",
                        "No power supply devices are configured or detected. Please check your configuration file."
                    }
                }
            } else if !selected_psu().is_empty() {
                // Control Panel
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                    // Output Control
                    div {
                        class: "space-y-4",
                        h3 {
                            class: "text-lg font-semibold text-gray-700",
                            "Output Control"
                        }

                        button {
                            class: if output_enabled() {
                                "w-full px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                            } else {
                                "w-full px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors"
                            },
                            onclick: move |_| toggle_output(),
                            if output_enabled() {
                                "Turn OFF"
                            } else {
                                "Turn ON"
                            }
                        }

                        div {
                            class: "text-center",
                            span {
                                class: if output_enabled() {
                                    "inline-block px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm"
                                } else {
                                    "inline-block px-3 py-1 bg-red-100 text-red-800 rounded-full text-sm"
                                },
                                if output_enabled() { "ON" } else { "OFF" }
                            }
                        }

                        button {
                            class: "w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors mt-2",
                            onclick: move |_| refresh_state(),
                            "Refresh State"
                        }
                    }

                    // Voltage and Current Control
                    div {
                        class: "space-y-4",
                        h3 {
                            class: "text-lg font-semibold text-gray-700",
                            "Settings"
                        }

                        // Voltage Control
                        div {
                            label {
                                class: "block text-sm font-medium text-gray-600 mb-1",
                                "Voltage (V):"
                            }
                            div {
                                class: "flex gap-2",
                                input {
                                    class: "flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "number",
                                    step: "0.1",
                                    min: "0",
                                    value: voltage(),
                                    oninput: move |evt| voltage.set(evt.value())
                                }
                                button {
                                    class: "px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 transition-colors",
                                    onclick: move |_| set_voltage_fn(),
                                    "Set"
                                }
                            }
                        }

                        // Current Control
                        div {
                            label {
                                class: "block text-sm font-medium text-gray-600 mb-1",
                                "Current Limit (A):"
                            }
                            div {
                                class: "flex gap-2",
                                input {
                                    class: "flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "number",
                                    step: "0.01",
                                    min: "0",
                                    value: current(),
                                    oninput: move |evt| current.set(evt.value())
                                }
                                button {
                                    class: "px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 transition-colors",
                                    onclick: move |_| set_current_fn(),
                                    "Set"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
