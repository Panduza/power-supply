use crate::client::PowerSupplyClient;

use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Props, Clone)]
pub struct ControlBoxProps {
    /// The PSU client for controlling the power supply
    pub psu_client: Option<Arc<Mutex<PowerSupplyClient>>>,
}

impl PartialEq for ControlBoxProps {
    fn eq(&self, other: &Self) -> bool {
        self.psu_client.is_some() == other.psu_client.is_some()
    }
}

#[component]
pub fn ControlBox(props: ControlBoxProps) -> Element {
    // Rendering the button
    rsx! {
        div {
            div { "ON" }
            button {
                "TOGGLE"
            }
            div {
                input {
                    r#type: "number",
                    step: "0.01",
                    min: "0",
                    placeholder: "0.00",
                    value: 5.0,
                }
                span { "V" }
            }
            button { "SET" }
            div {
                input {
                    r#type: "number",
                    step: "0.01",
                    min: "0",
                    placeholder: "0.00",
                    value: 5.0,
                }
                span { "A" }
            }
            button { "SET" }
        }
    }
}
