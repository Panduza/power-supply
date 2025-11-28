use anyhow::Result;
use bytes::Bytes;
use pza_power_supply_client::payload::{
    CurrentPayload, PowerState, PowerStatePayload, VoltagePayload,
};
use pza_power_supply_client::topics::Topics;
use pza_power_supply_client::PowerSupplyClient;
use pza_toolkit::rumqtt::broker::start_broker_in_thread;
use pza_toolkit::rumqtt::client::init_client;
use rumqttc::{Event, Packet, QoS};
use std::time::Duration;
use tokio::time::timeout;

use pza_toolkit::config::IPEndpointConfig;

/// Configuration for the test MQTT broker
const TEST_BROKER_ADDR: &str = "127.0.0.1";
const TEST_BROKER_PORT: u16 = 1884; // Different from default to avoid conflicts
const TEST_PSU_NAME: &str = "emulator";

/// Test power state command and response
#[tokio::test]
async fn mqtt() -> anyhow::Result<()> {
    // Start a test client
    let client = PowerSupplyClient::builder()
        .with_power_supply_name(TEST_PSU_NAME)
        .with_ip(IPEndpointConfig {
            addr: Some(TEST_BROKER_ADDR.to_string()),
            port: Some(TEST_BROKER_PORT),
        })
        .build()?;

    //
    let timeout = Duration::from_secs(5);

    // Test setting power state to ON
    client.enable_output_wait_ack(timeout.clone()).await?;
    client.disable_output_wait_ack(timeout.clone()).await?;
    client
        .set_voltage_wait_ack("12.5".to_string(), timeout.clone())
        .await?;
    client
        .set_current_wait_ack("2.4".to_string(), timeout.clone())
        .await?;

    Ok(())
}
