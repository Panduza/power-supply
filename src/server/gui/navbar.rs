use crate::server::gui::routes::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// Navigation bar component for the application
#[component]
pub fn NavBar() -> Element {
    // State for sidebar visibility
    let mut sidebar_visible = use_signal(|| true);

    // Toggle sidebar visibility
    let toggle_sidebar = move |_| {
        let current_state = *sidebar_visible.read();
        sidebar_visible.set(!current_state);
    };

    // Keyboard shortcut handler for sidebar toggle
    let on_key_down = move |event: Event<KeyboardData>| {
        // Toggle sidebar with Ctrl+B
        if event.modifiers().ctrl() && event.key() == Key::Character("b".to_string()) {
            let current_state = *sidebar_visible.read();
            sidebar_visible.set(!current_state);
        }
    };

    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // Rendering
    rsx! {
        div {
            class: "app-container",
            onkeydown: on_key_down,
            tabindex: 0,

            // Left sidebar
            aside {
                class: if *sidebar_visible.read() { "sidebar sidebar-visible" } else { "sidebar sidebar-hidden" },

                // Sidebar header
                div {
                    class: "sidebar-header",
                    div {
                        class: "sidebar-brand",
                        "Panduza"
                    }
                    button {
                        class: "sidebar-toggle",
                        onclick: toggle_sidebar,
                        title: "Hide sidebar (Ctrl+B)",
                        "⟨"
                    }
                }

                // Navigation menu
                nav {
                    class: "sidebar-nav",
                    ul {
                        class: "nav-list",
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Control,
                                class: "nav-link",
                                span {
                                    class: "nav-icon",
                                    "⚡"
                                }
                                span {
                                    class: "nav-text",
                                    "Control"
                                }
                            }
                        }
                        li {
                            class: "nav-item",
                            Link {
                                to: Route::Mcp,
                                class: "nav-link",
                                span {
                                    class: "nav-icon",
                                    "🔧"
                                }
                                span {
                                    class: "nav-text",
                                    "MCP"
                                }
                            }
                        }
                    }
                }
            }

            // Floating toggle button (when sidebar is hidden)
            if !*sidebar_visible.read() {
                button {
                    class: "floating-toggle",
                    onclick: toggle_sidebar,
                    title: "Show sidebar (Ctrl+B)",
                    "☰"
                }
            }

            // Main content area
            main {
                class: if *sidebar_visible.read() { "main-content main-with-sidebar" } else { "main-content main-full-width" },

                // Page content
                div {
                    class: "page-content",
                    Outlet::<Route> {}
                }
            }
        }
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++
}
