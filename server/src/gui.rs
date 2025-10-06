use dioxus::prelude::*;
use panduza_power_supply_client::{PowerSupplyClient, PowerSupplyClientBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

mod button_power;
use button_power::PowerButton;

const FAVICON: Asset = asset!("/assets/favicon.ico");
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
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100",

            // Modern header with status
            header {
                class: "bg-white/80 backdrop-blur-sm border-b border-slate-200/50 sticky top-0 z-10",
                div {
                    class: "container mx-auto px-6 py-4 flex justify-between items-center",
                    div {
                        class: "flex items-center space-x-3",
                        div {
                            class: "w-10 h-10 bg-gradient-to-r from-blue-500 to-indigo-600 rounded-xl flex items-center justify-center",
                            span {
                                class: "text-white text-xl font-bold",
                                "⚡"
                            }
                        }
                        div {
                            h1 {
                                class: "text-2xl font-bold bg-gradient-to-r from-slate-800 to-slate-600 bg-clip-text text-transparent",
                                "Panduza Power Supply"
                            }
                            p {
                                class: "text-sm text-slate-500",
                                "Control Center"
                            }
                        }
                    }
                    div {
                        class: "flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-green-50 to-emerald-50 border border-green-200/50 rounded-full",
                        div {
                            class: "w-2 h-2 bg-green-400 rounded-full animate-pulse"
                        }
                        span {
                            class: "text-sm font-medium text-green-700",
                            "{runtime_status}"
                        }
                    }
                }
            }

            main {
                class: "container mx-auto px-6 py-8",
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

    // Callbacks for PowerButton component
    let on_output_changed = move |enabled: bool| {
        output_enabled.set(enabled);
    };

    let on_status_message = move |message: String| {
        status_message.set(message);
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
            class: "max-w-4xl mx-auto space-y-6",

            // Status Card
            div {
                class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-6 shadow-xl shadow-slate-200/50",
                div {
                    class: "flex items-center justify-between",
                    h2 {
                        class: "text-xl font-semibold text-slate-700",
                        "System Status"
                    }
                    div {
                        class: {
                            if status_message().contains("Error") {
                                "px-4 py-2 bg-gradient-to-r from-red-50 to-rose-50 border border-red-200/50 rounded-full"
                            } else if status_message().contains("Connected") || status_message().contains("successfully") {
                                "px-4 py-2 bg-gradient-to-r from-green-50 to-emerald-50 border border-green-200/50 rounded-full"
                            } else {
                                "px-4 py-2 bg-gradient-to-r from-blue-50 to-sky-50 border border-blue-200/50 rounded-full"
                            }
                        },
                        span {
                            class: {
                                if status_message().contains("Error") {
                                    "text-sm font-medium text-red-700"
                                } else if status_message().contains("Connected") || status_message().contains("successfully") {
                                    "text-sm font-medium text-green-700"
                                } else {
                                    "text-sm font-medium text-blue-700"
                                }
                            },
                            "{status_message}"
                        }
                    }
                }
            }

            // PSU Selection Card
            div {
                class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-6 shadow-xl shadow-slate-200/50",
                div {
                    class: "flex items-center space-x-3 mb-4",
                    div {
                        class: "w-8 h-8 bg-gradient-to-r from-purple-500 to-pink-500 rounded-lg flex items-center justify-center",
                        span {
                            class: "text-white text-sm font-bold",
                            "🔌"
                        }
                    }
                    h3 {
                        class: "text-lg font-semibold text-slate-700",
                        "Device Selection"
                    }
                }
                label {
                    class: "block text-sm font-medium text-slate-600 mb-3",
                    "Choose Power Supply Device:"
                }
                select {
                    class: "w-full px-4 py-3 bg-white/50 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 focus:border-transparent transition-all duration-200 text-slate-700",
                    value: selected_psu(),
                    onchange: move |evt| {
                        selected_psu.set(evt.value());
                    },
                    option { value: "", "Select a device..." }
                    for name in psu_names() {
                        option { value: name.clone(), "{name}" }
                    }
                }
            }

            if psu_names().is_empty() {
                // No PSUs available message
                div {
                    class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-12 shadow-xl shadow-slate-200/50 text-center",
                    div {
                        class: "w-24 h-24 mx-auto mb-6 bg-gradient-to-r from-slate-100 to-slate-200 rounded-full flex items-center justify-center",
                        span {
                            class: "text-4xl text-slate-400",
                            "⚡"
                        }
                    }
                    h3 {
                        class: "text-2xl font-bold text-slate-700 mb-3",
                        "No Devices Found"
                    }
                    p {
                        class: "text-slate-500 text-lg mb-6",
                        "No power supply devices are configured or detected."
                    }
                    div {
                        class: "inline-flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-amber-50 to-orange-50 border border-amber-200/50 rounded-full",
                        span {
                            class: "text-amber-600 text-sm",
                            "💡"
                        }
                        span {
                            class: "text-sm text-amber-700 font-medium",
                            "Check your configuration file"
                        }
                    }
                }
            } else if !selected_psu().is_empty() {
                // Control Panel
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

                    // Output Control Card - Using PowerButton component
                    div {
                        class: "space-y-3",
                        PowerButton {
                            output_enabled: output_enabled(),
                            psu_client: psu_client(),
                            on_output_changed: on_output_changed,
                            on_status_message: on_status_message,
                        }

                        // Refresh button
                        div {
                            class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-4 shadow-xl shadow-slate-200/50",
                            button {
                                class: "w-full px-6 py-3 bg-gradient-to-r from-blue-500 to-sky-500 text-white font-medium rounded-xl hover:from-blue-600 hover:to-sky-600 transition-all duration-200 shadow-md hover:shadow-lg",
                                onclick: move |_| refresh_state(),
                                div {
                                    class: "flex items-center justify-center space-x-2",
                                    span { "🔄" }
                                    span { "Refresh State" }
                                }
                            }
                        }
                    }

                    // Voltage and Current Control Card
                    div {
                        class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-6 shadow-xl shadow-slate-200/50",
                        div {
                            class: "flex items-center space-x-3 mb-6",
                            div {
                                class: "w-8 h-8 bg-gradient-to-r from-orange-500 to-amber-500 rounded-lg flex items-center justify-center",
                                span {
                                    class: "text-white text-sm font-bold",
                                    "⚙️"
                                }
                            }
                            h3 {
                                class: "text-lg font-semibold text-slate-700",
                                "Power Settings"
                            }
                        }

                        div {
                            class: "space-y-6",

                            // Voltage Control
                            div {
                                class: "bg-gradient-to-r from-orange-50 to-amber-50 border border-orange-200/50 rounded-xl p-4",
                                label {
                                    class: "flex items-center space-x-2 text-sm font-semibold text-orange-700 mb-3",
                                    span { "⚡" }
                                    span { "Voltage Control" }
                                }
                                div {
                                    class: "flex items-center space-x-3",
                                    input {
                                        class: "flex-1 px-4 py-3 bg-white/70 border border-orange-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-orange-400 focus:border-transparent transition-all duration-200 text-slate-700 font-medium",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "0",
                                        placeholder: "0.0",
                                        value: voltage(),
                                        oninput: move |evt| voltage.set(evt.value())
                                    }
                                    span {
                                        class: "text-orange-600 font-semibold text-sm",
                                        "V"
                                    }
                                    button {
                                        class: "px-5 py-3 bg-gradient-to-r from-orange-500 to-amber-500 text-white font-semibold rounded-lg hover:from-orange-600 hover:to-amber-600 transition-all duration-200 shadow-md hover:shadow-lg transform hover:scale-105",
                                        onclick: move |_| set_voltage_fn(),
                                        "Set"
                                    }
                                }
                            }

                            // Current Control
                            div {
                                class: "bg-gradient-to-r from-purple-50 to-pink-50 border border-purple-200/50 rounded-xl p-4",
                                label {
                                    class: "flex items-center space-x-2 text-sm font-semibold text-purple-700 mb-3",
                                    span { "🔋" }
                                    span { "Current Limit" }
                                }
                                div {
                                    class: "flex items-center space-x-3",
                                    input {
                                        class: "flex-1 px-4 py-3 bg-white/70 border border-purple-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-400 focus:border-transparent transition-all duration-200 text-slate-700 font-medium",
                                        r#type: "number",
                                        step: "0.01",
                                        min: "0",
                                        placeholder: "0.00",
                                        value: current(),
                                        oninput: move |evt| current.set(evt.value())
                                    }
                                    span {
                                        class: "text-purple-600 font-semibold text-sm",
                                        "A"
                                    }
                                    button {
                                        class: "px-5 py-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white font-semibold rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all duration-200 shadow-md hover:shadow-lg transform hover:scale-105",
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
}
