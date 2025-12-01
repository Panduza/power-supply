mod mcp;
mod path;
mod power_supply;
mod tui;

use crate::server::config::mcp::McpConfig;
pub use power_supply::PowerSupplyConfig;
use pza_toolkit::config::MqttBrokerConfig;
use pza_toolkit::dioxus::logger::LoggerBuilder;
use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;
use tracing::Level;
use tracing_subscriber::field::debug;
pub use tui::TuiConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// TUI configuration
    pub tui: TuiConfig,

    /// MCP server configuration
    pub mcp: McpConfig,

    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,

    /// Power supply configurations, keyed by their unique identifiers
    pub runners: Option<HashMap<String, PowerSupplyConfig>>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        // Create a default power supply configuration for an emulator device
        let mut runners = HashMap::new();
        runners.insert(
            "emulator".to_string(),
            PowerSupplyConfig {
                model: "emulator".to_string(),
                description: None,
                security_min_voltage: Some(0.0),
                security_max_voltage: Some(30.0),
                security_min_current: Some(0.0),
                security_max_current: Some(5.0),
            },
        );

        // Return the default global configuration
        Self {
            tui: tui::TuiConfig {
                enable: Some(true),
                power_toggle_key: Some("p".to_string()),
            },
            mcp: McpConfig {
                enable: false,
                host: "127.0.0.1".to_string(),
                port: 50051,
            },
            broker: MqttBrokerConfig::default(),
            runners: Some(runners),
        }
    }
}

impl ServerConfig {
    /// Load the global configuration from the configuration file
    ///
    pub fn from_user_file() -> anyhow::Result<Self> {
        let config_path = path::server_config_file()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine server configuration file path"))?;

        pza_toolkit::config::read_config::<ServerConfig>(&config_path)
    }

    /// List MCP server URLs from the configuration
    ///
    fn list_mcp_servers_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();

        if let Some(runners) = &self.runners {
            for (name, config) in runners {
                let url = format!(
                    "http://{}:{}/{}/{}",
                    self.mcp.host,
                    self.mcp.port,
                    pza_power_supply_client::SERVER_TYPE_NAME,
                    name
                );
                urls.push(url);
            }
        }

        urls
    }

    /// List MCP server URLs as a JSON string
    fn list_mcp_servers_urls_as_json_string(&self) -> String {
        let urls = self.list_mcp_servers_urls();
        serde_json::to_string_pretty(&urls).unwrap_or_else(|_| "[]".to_string())
    }

    /// Print MCP server URLs to stdout
    pub fn print_mcp_servers_urls(&self) {
        let urls_json = self.list_mcp_servers_urls_as_json_string();
        println!("{}", urls_json);
    }

    /// Apply service overrides from CLI arguments
    ///
    pub fn apply_overrides(mut self, overrides: &crate::server::cli::ServicesOverrides) -> Self {
        if self.tui.enable.is_none() {
            self.tui.enable = Some(true);
        }
        if overrides.no_mcp {
            self.mcp.enable = false;
        }
        if overrides.no_tui {
            self.tui.enable = Some(false);
        }
        if overrides.no_runners {
            self.runners = None;
        }
        self
    }

    /// Get the names of all configured runners
    pub fn runner_names(&self) -> Vec<String> {
        match &self.runners {
            Some(runners) => runners.keys().cloned().collect(),
            None => Vec::new(),
        }
    }

    /// Determine if tracing should be enabled based on TUI configuration
    pub fn should_enable_tracing(&self) -> bool {
        // Enable tracing if TUI is disabled
        !self.tui.enable.unwrap_or(false)
    }

    /// Setup tracing based on the configuration
    pub fn setup_tracing(self) -> Self {
        if self.should_enable_tracing() {
            LoggerBuilder::default()
                .with_level(Level::TRACE)
                // .display_target(true)
                .filter_rumqttd()
                .filter_dioxus_core()
                .filter_dioxus_signals()
                .filter_warnings()
                .build()
                .expect("failed to init logger");
        }
        self
    }

    /// Trace the current configuration for debugging purposes
    pub fn trace_config(self) -> Self {
        debug!("Configuration file path: {:?}", path::server_config_file());
        debug!("Configuration after overrides: {:?}", self);
        self
    }
}
