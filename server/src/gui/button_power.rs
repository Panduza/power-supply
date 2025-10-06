use dioxus::prelude::*;
use panduza_power_supply_client::PowerSupplyClient;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Props, Clone)]
pub struct PowerButtonProps {
    /// Whether the output is currently enabled
    pub output_enabled: bool,
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
    /// Callback when the output state changes
    pub on_output_changed: EventHandler<bool>,
    /// Callback when there's a status message to display
    pub on_status_message: EventHandler<String>,
}

impl PartialEq for PowerButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.output_enabled == other.output_enabled
            && self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn PowerButton(props: PowerButtonProps) -> Element {
    // Toggle output enable/disable
    let toggle_output = move || {
        if let Some(client_arc) = props.psu_client.clone() {
            let enabled = props.output_enabled;
            let on_output_changed = props.on_output_changed.clone();
            let on_status_message = props.on_status_message.clone();

            spawn(async move {
                let client = client_arc.lock().await;
                let result = if enabled {
                    client.disable_output().await
                } else {
                    client.enable_output().await
                };

                match result {
                    Ok(()) => {
                        on_output_changed.call(!enabled);
                        on_status_message.call("Output toggled successfully".to_string());
                    }
                    Err(e) => {
                        on_status_message.call(format!("Error toggling output: {}", e));
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "bg-white/70 backdrop-blur-sm border border-white/20 rounded-2xl p-6 shadow-xl shadow-slate-200/50",
            div {
                class: "flex items-center space-x-3 mb-6",
                div {
                    class: "w-8 h-8 bg-gradient-to-r from-green-500 to-emerald-500 rounded-lg flex items-center justify-center",
                    span {
                        class: "text-white text-sm font-bold",
                        "🔋"
                    }
                }
                h3 {
                    class: "text-lg font-semibold text-slate-700",
                    "Power Output"
                }
            }

            // Status indicator
            div {
                class: {
                    let base = "mb-6 p-4 rounded-xl text-center ";
                    let color_classes = if props.output_enabled {
                        "bg-gradient-to-r from-green-50 to-emerald-50 border border-green-200/50"
                    } else {
                        "bg-gradient-to-r from-red-50 to-rose-50 border border-red-200/50"
                    };
                    format!("{}{}", base, color_classes)
                },
                div {
                    class: "flex items-center justify-center space-x-2 mb-2",
                    div {
                        class: {
                            if props.output_enabled {
                                "w-3 h-3 bg-green-400 rounded-full animate-pulse"
                            } else {
                                "w-3 h-3 bg-red-400 rounded-full"
                            }
                        }
                    }
                    span {
                        class: {
                            if props.output_enabled {
                                "text-2xl font-bold text-green-700"
                            } else {
                                "text-2xl font-bold text-red-700"
                            }
                        },
                        if props.output_enabled { "ON" } else { "OFF" }
                    }
                }
                p {
                    class: {
                        if props.output_enabled {
                            "text-sm text-green-600"
                        } else {
                            "text-sm text-red-600"
                        }
                    },
                    if props.output_enabled { "Output is active" } else { "Output is disabled" }
                }
            }

            // Control button
            button {
                class: {
                    if props.output_enabled {
                        "w-full px-6 py-4 bg-gradient-to-r from-red-500 to-rose-500 text-white font-semibold rounded-xl hover:from-red-600 hover:to-rose-600 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-[1.02]"
                    } else {
                        "w-full px-6 py-4 bg-gradient-to-r from-green-500 to-emerald-500 text-white font-semibold rounded-xl hover:from-green-600 hover:to-emerald-600 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-[1.02]"
                    }
                },
                onclick: move |_| toggle_output(),
                div {
                    class: "flex items-center justify-center space-x-2",
                    span {
                        class: "text-lg",
                        if props.output_enabled { "🔌" } else { "⚡" }
                    }
                    span {
                        if props.output_enabled { "Turn OFF" } else { "Turn ON" }
                    }
                }
            }
        }
    }
}
