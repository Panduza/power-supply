use crate::config::GlobalConfig;
use config::Map;
use config::Value;
use rumqttd::Broker;
use rumqttd::Config;
use tracing::info;

/// Start the MQTT broker in a separate thread
pub fn start(config: &GlobalConfig) -> std::thread::JoinHandle<()> {
    //
    // info
    info!("----- SERVICE : START BROKER -----");

    let host: &str = config.broker.host.as_ref();
    let port = config.broker.port;
    let listen_addr = format!("{}:{}", host, port);

    let mut router: std::collections::HashMap<String, Value> = Map::new();
    router.insert("id".to_string(), Value::new(None, 0));
    router.insert("max_connections".to_string(), Value::new(None, 20480));
    router.insert(
        "max_outgoing_packet_count".to_string(),
        Value::new(None, 200),
    );
    router.insert("max_segment_size".to_string(), Value::new(None, 104857600));
    router.insert("max_segment_count".to_string(), Value::new(None, 10));

    let mut server_connections: std::collections::HashMap<String, config::Value> = Map::new();
    server_connections.insert("connection_timeout_ms".to_string(), Value::new(None, 60000));
    server_connections.insert("max_payload_size".to_string(), Value::new(None, 2000480));
    server_connections.insert("max_inflight_count".to_string(), Value::new(None, 20480));
    server_connections.insert("dynamic_filters".to_string(), Value::new(None, true));

    let mut server: std::collections::HashMap<String, Value> = Map::new();
    server.insert("name".to_string(), Value::new(None, "v4-1"));
    server.insert("listen".to_string(), Value::new(None, listen_addr.clone()));
    server.insert("next_connection_delay_ms".to_string(), Value::new(None, 1));
    server.insert(
        "connections".to_string(),
        Value::new(None, server_connections),
    );

    // see docs of config crate to know more
    let config = config::Config::builder()
        .set_default("id", 0)
        .unwrap()
        .set_default("router", router)
        .unwrap()
        .set_default("v4.1", server)
        .unwrap()
        .build()
        .unwrap();

    //
    // this is where we deserialize it into Config
    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);

    //
    // start broker
    info!("Broker listen on: {}", listen_addr);
    let _jh = std::thread::spawn(move || {
        broker.start().unwrap();
        println!("BROKER STOPPPED !!!!!!!!!!!!!!!!!");
    });

    return _jh;
}
