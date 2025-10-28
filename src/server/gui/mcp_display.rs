use arboard::Clipboard;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct McpDisplayProps {
    /// The name of the power supply instance
    pub psu_name: String,
}

#[component]
pub fn McpDisplay(props: McpDisplayProps) -> Element {
    let mcp_url = format!("http://127.0.0.1:3000/power-supply/{}", props.psu_name);

    let mcp_url_bis = mcp_url.clone();

    // Function to copy URL to clipboard
    let copy_to_clipboard = move || {
        let url = mcp_url.clone();
        spawn(async move {
            match Clipboard::new() {
                Ok(mut clipboard) => match clipboard.set_text(url.clone()) {
                    Ok(()) => {
                        tracing::info!("MCP URL copied to clipboard: {}", url);
                    }
                    Err(e) => {
                        tracing::error!("Failed to copy to clipboard: {}", e);
                    }
                },
                Err(e) => {
                    tracing::error!("Failed to access clipboard: {}", e);
                }
            }
        });
    };

    rsx! {
        div {
            class: "mcp-display-container",

            div {
                class: "mcp-display-label",
                "MCP Server URL"
            }

            div {
                class: "mcp-url-container",

                div {
                    class: "mcp-url-text",
                    "{mcp_url_bis}"
                }

                button {
                    class: "mcp-copy-button",
                    onclick: move |_| copy_to_clipboard(),
                    "Copy"
                }
            }
        }
    }
}
