use crate::client::client::PowerSupplyClient;
use pza_toolkit::config::IPEndpointConfig;
use pza_toolkit::rumqtt;
use pza_toolkit::rumqtt::client::RumqttCustomAsyncClient;

#[derive(Default)]
/// Builder pattern for creating PowerSupplyClient instances
pub struct PowerSupplyClientBuilder {
    /// Name of the power supply unit
    pub psu_name: Option<String>,

    /// MQTT broker configuration
    pub ip: Option<IPEndpointConfig>,
}

impl PowerSupplyClientBuilder {
    // ------------------------------------------------------------------------

    /// Create a new builder from broker configuration
    pub fn with_ip(mut self, ip: IPEndpointConfig) -> Self {
        self.ip = Some(ip);
        self
    }

    // ------------------------------------------------------------------------

    /// Set the power supply name for the client
    pub fn with_power_supply_name<A: Into<String>>(mut self, name: A) -> Self {
        self.psu_name = Some(name.into());
        self
    }

    // ------------------------------------------------------------------------

    /// Build the PowerSupplyClient instance
    pub fn build(self) -> PowerSupplyClient {
        let (client, event_loop) = rumqtt::client::init_client("power-supply");

        PowerSupplyClient::new_with_client(
            self.psu_name.unwrap(),
            RumqttCustomAsyncClient::new(
                client,
                rumqttc::QoS::AtMostOnce,
                true,
                "power-supply".to_string(),
            ),
            event_loop,
        )
    }
}
