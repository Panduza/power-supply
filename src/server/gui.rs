use crate::{client::PowerSupplyClient, server::ServerState, SERVER_STATE_STORAGE};
use dioxus::prelude::*;
use pza_toolkit::config::IPEndpointConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use tracing::trace;
use tracing::warn;

mod control_box;
mod mcp_display;
use control_box::ControlBox;
use mcp_display::McpDisplay;

const CSS_MAIN: Asset = asset!("/assets/css/main.css");
const CSS_CONTROL_BOX: Asset = asset!("/assets/css/control_box.css");
const CSS_MCP_DISPLAY: Asset = asset!("/assets/css/mcp_display.css");
const CSS_BUTTON_CONFIG: Asset = asset!("/assets/css/control_box/button_config.css");
const CSS_BUTTON_POWER: Asset = asset!("/assets/css/control_box/button_power.css");
const CSS_CURRENT_SETTER: Asset = asset!("/assets/css/control_box/current_setter.css");
const CSS_INSTANCE_SELECTOR: Asset = asset!("/assets/css/control_box/instance_selector.css");
const CSS_VOLTAGE_SETTER: Asset = asset!("/assets/css/control_box/voltage_setter.css");

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
    let on_instance_changed = move |selected_instance: String| {
        trace!("Create a new client for instance: {}", selected_instance);

        // Update the selected instance signal
        s_selected.set(Some(selected_instance.clone()));

        // Get the current address from the signal
        if let Some(addr) = s_addr.read().clone() {
            let client = PowerSupplyClient::builder()
                .with_ip(addr)
                .with_power_supply_name(selected_instance)
                .build();
            if let Ok(client) = client {
                s_client.set(Some(Arc::new(Mutex::new(client))));
            } else {
                warn!("Failed to create PowerSupplyClient");
            }
        } else {
            warn!("Address not available yet");
        }
    };

    rsx! {
        document::Stylesheet { href: CSS_MAIN }
        document::Stylesheet { href: CSS_CONTROL_BOX }
        document::Stylesheet { href: CSS_MCP_DISPLAY }
        document::Stylesheet { href: CSS_BUTTON_CONFIG }
        document::Stylesheet { href: CSS_BUTTON_POWER }
        document::Stylesheet { href: CSS_CURRENT_SETTER }
        document::Stylesheet { href: CSS_INSTANCE_SELECTOR }
        document::Stylesheet { href: CSS_VOLTAGE_SETTER }

        div {
            class: "main-container",

            ControlBox {
                instance_client: s_client.read().clone(),
                selected_instance: s_selected.read().clone(),
                instances_names: s_names.read().clone(),
                on_instance_changed: on_instance_changed,
            }

            if let Some(selected) = s_selected.read().clone() {
                McpDisplay {
                    psu_name: selected.clone()
                }
            }

        }
    }
}
