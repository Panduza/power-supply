use arboard::Clipboard;
use dioxus::prelude::*;
use tracing::{error, info};

#[component]
pub fn ConfigButton() -> Element {
    // Function to copy config path to clipboard
    let copy_to_clipboard = move || {
        spawn(async move {
            // Get the configuration file path
            if let Some(config_path) = crate::path::server_config_file() {
                let path_str = config_path.to_string_lossy().to_string();
                
                // Copy to clipboard
                match Clipboard::new() {
                    Ok(mut clipboard) => {
                        if let Err(e) = clipboard.set_text(&path_str) {
                            error!("Failed to copy to clipboard: {}", e);
                        } else {
                            info!("Configuration path copied to clipboard: {}", path_str);
                        }
                    }
                    Err(e) => {
                        error!("Failed to access clipboard: {}", e);
                    }
                }
            } else {
                error!("Failed to get configuration file path");
            }
        });
    };

    rsx! {
        div {
            class: "config-button-container",

            // Label
            div {
                class: "config-button-label",
                "CONFIG"
            }

            // Button
            button {
                class: "config-button",
                onclick: move |_| copy_to_clipboard(),
                "Copy Path"
            }
        }
    }
}
