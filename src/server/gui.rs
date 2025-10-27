use crate::{client::PowerSupplyClient, server::ServerState, SERVER_STATE_STORAGE};
use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use pza_toolkit::config::IPEndpointConfig;
use std::sync::Arc;
use tokio::sync::Mutex;

mod config_button;

mod control_box;
use control_box::ControlBox;

static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_asset_data_url(filename: &str) -> String {
    if let Some(file) = ASSETS_DIR.get_file(filename) {
        let contents = file.contents();
        let mime_type = match filename.split('.').last().unwrap_or("") {
            "css" => "text/css",
            "ico" => "image/x-icon",
            "svg" => "image/svg+xml",
            _ => "application/octet-stream",
        };
        format!(
            "data:{};base64,{}",
            mime_type,
            general_purpose::STANDARD.encode(contents)
        )
    } else {
        String::new()
    }
}

#[component]
pub fn Gui() -> Element {
    // server_state: ServerState
    // Provide server state context

    // Inject server state into context
    let server_state: Arc<ServerState> = SERVER_STATE_STORAGE.get().unwrap().clone();
    use_context_provider(|| server_state.clone());

    let mqtt_addr: Signal<Option<IPEndpointConfig>> = use_signal(|| None);

    use_effect(move || {
        let server_state = server_state.clone();
        let mut mqtt_addr = mqtt_addr.clone();

        spawn(async move {
            let addr = server_state
                .as_ref()
                .server_config
                .lock()
                .await
                .broker
                .tcp
                .clone()
                .expect("No broker IP configured");
            mqtt_addr.set(Some(addr));
        });
    });

    let mut instance_client: Signal<Option<Arc<Mutex<PowerSupplyClient>>>> = use_signal(|| None);

    let mqtt_addr_value = mqtt_addr.read().clone();

    if let Some(mqtt_addr) = mqtt_addr_value {
        rsx! {
            document::Link { rel: "icon", href: get_asset_data_url("favicon.ico") }
            document::Link { rel: "stylesheet", href: get_asset_data_url("tailwind.css") }
            document::Link { rel: "stylesheet", href: get_asset_data_url("main.css") }
            document::Link { rel: "stylesheet", href: get_asset_data_url("button_power.css") }


            div {
                class: "main-container",

                header {
                    h1 {
                        "Panduza Power Supply"
                    }
                }

                main {
                    ControlBox {
                        instance_client: instance_client.read().clone(),
                        selected_instance: "".to_string(),
                        instances_names: vec![],
                        on_instance_changed: move |selected_instance : String| {

                            let client = PowerSupplyClient::builder()
                                .with_ip(mqtt_addr.clone())
                                .with_power_supply_name(selected_instance.clone())
                                .build();

                            instance_client.set(Some(Arc::new(Mutex::new(client))));
                        },
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "main-container",
                "Loading configuration..."
            }
        }
    }

    // rsx! {
    //     document::Link { rel: "icon", href: get_asset_data_url("favicon.ico") }
    //     document::Link { rel: "stylesheet", href: get_asset_data_url("tailwind.css") }
    //     document::Link { rel: "stylesheet", href: get_asset_data_url("main.css") }
    //     document::Link { rel: "stylesheet", href: get_asset_data_url("button_power.css") }

    //     div {
    //         class: "main-container",

    //         header {
    //             h1 {
    //                 "Panduza Power Supply"
    //             }
    //         }

    //         main {
    //             ControlBox {
    //                 instance_client: instance_client.read().clone(),
    //                 selected_instance: "".to_string(),
    //                 instances_names: vec![],
    //                 on_instance_changed: |selected_instance : String| {

    //                     // mqtt_addr.read().clone()

    //                     let client = PowerSupplyClient::builder().with_ip().with_power_supply_name(selected_instance.clone()).build();

    //                     // instance_client
    //                 },
    //             }
    //         }
    //     }
    // }
}

// #[component]
// pub fn PowerSupplyControl() -> Element {
//     let app_state = use_context::<crate::AppState>();
//     let mut selected_psu = use_signal(|| String::new());
//     let mut status_message = use_signal(|| "Ready".to_string());
//     let mut psu_names = use_signal(|| Vec::<String>::new());

//     // Load PSU names from app state
//     {
//         let psu_names_arc = app_state.psu_names.clone();
//         use_effect(move || {
//             let psu_names_arc = psu_names_arc.clone();
//             spawn(async move {
//                 loop {
//                     let names = psu_names_arc.lock().await;
//                     if !names.is_empty() {
//                         psu_names.set(names.clone());
//                         if selected_psu().is_empty() && !names.is_empty() {
//                             selected_psu.set(names[0].clone());
//                         }
//                         break;
//                     }
//                     drop(names);
//                     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//                 }
//             });
//         });
//     }

//     // Create PSU client when selection changes
//     {
//         let broker_config_arc = app_state.broker_config.clone();
//         use_effect(move || {
//             let selected = selected_psu();

//             if !selected.is_empty() {
//                 let broker_config_arc = broker_config_arc.clone();
//                 spawn(async move {
//                     let broker_config = broker_config_arc.lock().await;
//                     if let Some(config) = broker_config.as_ref() {
//                         let client = PowerSupplyClientBuilder::from_broker_config(config.clone())
//                             .with_power_supply_name(selected.clone())
//                             .build();

//                         psu_client.set(Some(Arc::new(Mutex::new(client))));
//                         status_message.set(format!("Connected to {}", selected));
//                     }
//                 });
//             }
//         });
//     }

//     rsx! {
//         div {
//             class: "content-wrapper",

//             // Configuration Button
//             ConfigButton {}

//             if psu_names().is_empty() {
//                 // No PSUs available message
//                 div {
//                     class: "glass-card text-center",

//                     h3 {
//                         class: "component-title text-2xl mb-3",
//                         "No Devices Found"
//                     }
//                     p {
//                         class: "text-slate-500 text-lg mb-6",
//                         "No power supply devices are configured or detected."
//                     }
//                     div {
//                         class: "status-message info",
//                         span { "💡 Check your configuration file" }
//                     }
//                 }
//             } else if !selected_psu().is_empty() {
//                 // Control Panel
//                 div {
//                     class: "control-grid",

//                     // Output Control Card - Using PowerButton component
//                     div {
//                         // PowerButton {
//                         //     psu_client: psu_client(),
//                         // }
//                     }

//                     // Voltage and Current Control Card
//                     div {
//                         class: "glass-card",

//                         div {
//                             class: "space-y-6",

//                             // // Voltage Control Component
//                             // VoltageSetter {
//                             //     voltage: voltage(),
//                             //     psu_client: psu_client(),
//                             //     on_voltage_changed: on_voltage_changed,
//                             //     on_status_message: on_status_message,
//                             // }

//                             // // Current Control Component
//                             // CurrentSetter {
//                             //     current: current(),
//                             //     psu_client: psu_client(),
//                             //     on_current_changed: on_current_changed,
//                             //     on_status_message: on_status_message,
//                             // }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
