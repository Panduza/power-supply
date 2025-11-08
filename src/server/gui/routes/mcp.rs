use dioxus::prelude::*;
use std::sync::Arc;
use tracing::debug;

use crate::{server::ServerState, SERVER_STATE_STORAGE};
use crate::server::gui::mcp_display::McpDisplay;

/// MCP (Model Context Protocol) page component
#[component]
pub fn Mcp() -> Element {
    // Inject server state into context
    use_context_provider(|| {
        SERVER_STATE_STORAGE
            .get()
            .expect("Failed to get server state")
            .clone()
    });

    // Signals
    let mut s_selected: Signal<Option<String>> = use_signal(|| None);
    let mut s_names: Signal<Option<Vec<String>>> = use_signal(|| None);

    // Coroutine to load configuration from server state
    let _coro: Coroutine<()> = use_coroutine({
        move |_rx| async move {
            // Get server state from context
            let server_state: Arc<ServerState> = use_context();

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
                s_selected.set(Some(first_name));
            }
        }
    });

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // Rendering
    rsx! {
        div {
            class: "mcp-page",

            h1 {
                class: "page-title",
                "Model Context Protocol Interface"
            }

            if let Some(names) = s_names.read().clone() {
                if !names.is_empty() {
                    div {
                        class: "instance-selector",
                        label {
                            "for": "psu-selector",
                            "Select Power Supply Instance:"
                        }
                        select {
                            id: "psu-selector",
                            value: s_selected.read().clone().unwrap_or_default(),
                            onchange: move |evt| {
                                s_selected.set(Some(evt.value().clone()));
                            },
                            for name in names {
                                option {
                                    value: name.clone(),
                                    selected: s_selected.read().as_ref() == Some(&name),
                                    {name.clone()}
                                }
                            }
                        }
                    }

                    if let Some(selected) = s_selected.read().clone() {
                        McpDisplay {
                            psu_name: selected
                        }
                    }
                } else {
                    div {
                        class: "no-instances",
                        "No power supply instances available"
                    }
                }
            } else {
                div {
                    class: "loading",
                    "Loading instances..."
                }
            }
        }
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
}
