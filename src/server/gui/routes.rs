use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Control,
    #[route("/mcp")]
    Mcp,
}
