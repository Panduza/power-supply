use crate::{client::PowerSupplyClient, server::ServerState, SERVER_STATE_STORAGE};
use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use include_dir::{include_dir, Dir};
use pza_toolkit::config::IPEndpointConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

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
    // Inject server state into context
    use_context_provider(|| {
        SERVER_STATE_STORAGE
            .get()
            .expect("Failed to get server state")
            .clone()
    });

    // Signals
    let mut s_addr: Signal<Option<IPEndpointConfig>> = use_signal(|| None);
    let mut s_selected: Signal<Option<String>> = use_signal(|| None);
    let mut s_names: Signal<Option<Vec<String>>> = use_signal(|| None);
    let mut s_client: Signal<Option<Arc<Mutex<PowerSupplyClient>>>> = use_signal(|| None);

    // Coroutine to load configuration from server state
    let _coro: Coroutine<()> = use_coroutine({
        move |_rx| async move {
            // Get server state from context
            let server_state: Arc<ServerState> = use_context();

            // Sleep for 5 seconds
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            s_addr.set(server_state.server_config.lock().await.broker.tcp.clone());

            // Load instance names from server state
            let names: Vec<String> = server_state
                .instances
                .lock()
                .await
                .keys()
                .cloned()
                .collect();
            debug!("Loaded instances names: {:?}", names);
            s_names.set(Some(names.clone()));

            // If selected_instance is None and instances_names is not empty,
            // set selected_instance to the first element
            if s_selected.read().is_none() && !names.is_empty() {
                let first_name = names[0].clone();
                debug!("Setting selected_instance to: {:?}", first_name);
                s_selected.set(Some(first_name.clone()));

                let client = PowerSupplyClient::builder()
                    .with_ip(s_addr.read().clone().expect("address not set").clone())
                    .with_power_supply_name(
                        s_selected
                            .read()
                            .clone()
                            .expect("selected instance not set"),
                    )
                    .build();

                if let Ok(client) = client {
                    s_client.set(Some(Arc::new(Mutex::new(client))));
                } else {
                    warn!("Failed to create PowerSupplyClient");
                }
            }
        }
    });

    // Create the callback closure that can mutate instance_client
    let on_instance_changed = {
        let mqtt_addr_value = s_addr.read().clone();
        move |selected_instance: String| {
            trace!("Create a new client for instance: {}", selected_instance);
            let client = PowerSupplyClient::builder()
                .with_ip(mqtt_addr_value.clone().expect("address not set").clone())
                .with_power_supply_name(selected_instance.clone())
                .build();
            if let Ok(client) = client {
                s_client.set(Some(Arc::new(Mutex::new(client))));
            } else {
                warn!("Failed to create PowerSupplyClient");
            }
        }
    };

    let instance_client = s_client.read().clone();
    let instances_names = s_names.read().clone();
    let selected_instance = s_selected.read().clone();

    rsx! {
        div {
            class: "main-container",
            ControlBox {
                instance_client: instance_client.clone(),
                selected_instance: selected_instance.clone(),
                instances_names: instances_names.clone(),
                on_instance_changed: on_instance_changed,
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

    //             if let (Some(mqtt_addr), Some(instances_names), Some(selected_instance), Some(i_client)) = (mqtt_addr_value, instances_names_value, selected_instance_value, instance_client_value) {

    //             } else {
    //                 div {
    //                     "Loading configuration..."
    //                 }
    //             }
    //         }
    //     }
    // }
}
