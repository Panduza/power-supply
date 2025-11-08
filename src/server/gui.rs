use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod control_box;
mod mcp_display;
mod navbar;
mod routes;

use routes::Route;

const CSS_MAIN: Asset = asset!("/assets/css/main.css");
const CSS_NAVBAR: Asset = asset!("/assets/css/navbar.css");
const CSS_CONTROL_BOX: Asset = asset!("/assets/css/control_box.css");
const CSS_MCP_DISPLAY: Asset = asset!("/assets/css/mcp_display.css");
const CSS_BUTTON_POWER: Asset = asset!("/assets/css/control_box/button_power.css");
const CSS_CURRENT_SETTER: Asset = asset!("/assets/css/control_box/current_setter.css");
const CSS_INSTANCE_SELECTOR: Asset = asset!("/assets/css/control_box/instance_selector.css");
const CSS_VOLTAGE_SETTER: Asset = asset!("/assets/css/control_box/voltage_setter.css");

#[component]
pub fn Gui() -> Element {
    rsx! {
        document::Stylesheet { href: CSS_MAIN }
        document::Stylesheet { href: CSS_NAVBAR }
        document::Stylesheet { href: CSS_CONTROL_BOX }
        document::Stylesheet { href: CSS_MCP_DISPLAY }
        document::Stylesheet { href: CSS_BUTTON_POWER }
        document::Stylesheet { href: CSS_CURRENT_SETTER }
        document::Stylesheet { href: CSS_INSTANCE_SELECTOR }
        document::Stylesheet { href: CSS_VOLTAGE_SETTER }

        Router::<Route> {}
    }
}
