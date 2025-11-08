use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod control;
pub mod mcp;

pub use control::Control;
pub use mcp::Mcp;

use super::navbar::NavBar;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Control,
    #[route("/mcp")]
    Mcp,
}
