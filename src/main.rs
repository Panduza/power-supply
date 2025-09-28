mod broker;
mod config;
mod drivers;
mod factory;
mod path;
mod runner;

use tracing::{debug, Level};

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    // Get user configuration
    let config = config::GlobalConfig::from_user_file();
    debug!("Loaded configuration: {:?}", config);

    // Create factory
    let factory = factory::Factory::new();
    debug!("Factory initialized with drivers: {:?}", factory.map.keys());

    // Start MQTT broker
    let _broker_handle = broker::start(&config);

    // Initialize devices
    if let Some(devices) = &config.devices {
        for (name, config) in devices {
            let instance = factory
                .instanciate_driver(config.clone())
                .unwrap_or_else(|err| {
                    panic!("Failed to create driver for device '{}': {}", name, err)
                });

            // Runner::new(name.clone(), instance).start();
        }
    }

    // Launch dioxus app
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
