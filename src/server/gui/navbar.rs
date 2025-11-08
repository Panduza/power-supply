use crate::server::gui::routes::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// Navigation bar component for the application
#[component]
pub fn NavBar() -> Element {
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // Rendering
    rsx! {
        nav {
            class: "navbar",
            div {
                class: "navbar-brand",
                "Panduza Power Supply"
            }
            ul {
                class: "navbar-nav",
                li {
                    class: "nav-item",
                    Link {
                        to: Route::Control,
                        class: "nav-link",
                        "Control"
                    }
                }
                li {
                    class: "nav-item",
                    Link {
                        to: Route::Mcp,
                        class: "nav-link",
                        "MCP"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
}
