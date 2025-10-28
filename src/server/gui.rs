use crate::{client::PowerSupplyClient, server::ServerState, SERVER_STATE_STORAGE};
use base64::{engine::general_purpose, Engine as _};
use dioxus::{html::select, prelude::*};
use include_dir::{include_dir, Dir};
use pza_toolkit::config::IPEndpointConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

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

    // Signals
    let mqtt_addr: Signal<Option<IPEndpointConfig>> = use_signal(|| None);
    let selected_instance: Signal<Option<String>> = use_signal(|| None);
    let instances_names: Signal<Option<Vec<String>>> = use_signal(|| None);
    let mut instance_client: Signal<Option<Arc<Mutex<PowerSupplyClient>>>> = use_signal(|| None);

    // Effects
    use_effect({
        let s = server_state.clone();
        move || {
            load_mqtt_addr_from_server_config(s.clone(), mqtt_addr.clone());
        }
    });
    use_effect(move || {
        let s = server_state.clone();
        load_instances_choices(
            s.clone(),
            instances_names.clone(),
            selected_instance.clone(),
        );
    });

    let instance_client_value = instance_client.read().clone();
    let mqtt_addr_value = mqtt_addr.read().clone();
    let instances_names_value = instances_names.read().clone();
    let selected_instance_value = selected_instance.read().clone();

    // Create the callback closure that can mutate instance_client
    let mqtt_addr_value_2 = mqtt_addr_value.clone();
    let on_instance_changed = move |selected_instance: String| {
        let client = PowerSupplyClient::builder()
            .with_ip(mqtt_addr_value_2.clone().expect("address not set").clone())
            .with_power_supply_name(selected_instance.clone())
            .build();

        instance_client.set(Some(Arc::new(Mutex::new(client))));
    };

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

                if let (Some(mqtt_addr), Some(instances_names), Some(selected_instance), Some(i_client)) = (mqtt_addr_value, instances_names_value, selected_instance_value, instance_client_value) {

                    ControlBox {
                        instance_client: i_client.clone(),
                        selected_instance: selected_instance.clone(),
                        instances_names: instances_names.clone(),
                        on_instance_changed: on_instance_changed,
                    }
                } else {
                    div {
                        "Loading configuration..."
                    }
                }
            }
        }
    }
}

/// Component initialization function
fn load_mqtt_addr_from_server_config(
    server_state: Arc<ServerState>,
    mut mqtt_addr: Signal<Option<IPEndpointConfig>>,
) {
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
}

/// Component initialization function
fn load_instances_choices(
    server_state: Arc<ServerState>,
    mut instances_names: Signal<Option<Vec<String>>>,
    mut selected_instance: Signal<Option<String>>,
) {
    spawn(async move {
        // Load instance names from server state
        let names: Vec<String> = server_state
            .as_ref()
            .instances
            .lock()
            .await
            .keys()
            .cloned()
            .collect();
        debug!("Loaded instances names: {:?}", names);
        instances_names.set(Some(names.clone()));

        // If selected_instance is None and instances_names is not empty,
        // set selected_instance to the first element
        if selected_instance.read().is_none() && !names.is_empty() {
            debug!("Setting selected_instance to: {:?}", names[0]);
            selected_instance.set(Some(names[0].clone()));
        }
    });
}
