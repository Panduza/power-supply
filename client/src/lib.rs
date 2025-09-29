mod config;
mod path;



pub struct Client {
    mqtt_client: AsyncClient,
}

impl Client {

    pub fn new() -> Self {
        // Initialize MQTT client
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            "localhost",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, mut event_loop) = AsyncClient::new(mqttoptions, 100);

        Self { mqtt_client: client }
    }

}
