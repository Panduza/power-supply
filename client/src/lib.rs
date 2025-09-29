mod config;
mod path;


pub struct ClientBuilder {


}

impl ClientBuilder {
 
    pub fn from_user_config_file() -> Self {
        Self {}
    }
    
    pub fn build(self) -> Client {
        Client::new()
    }
}


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


                let task_handler = tokio::spawn(async move {
            // // Subscribe to all relevant topics
            // Self::subscribe_to_all(
            //     client.clone(),
            //     vec![
            //         &runner.topic_control_oe_cmd,
            //         &runner.topic_control_voltage_cmd,
            //         &runner.topic_control_current_cmd,
            //         &runner.topic_measure_voltage_refresh_freq,
            //         &runner.topic_measure_current_refresh_freq,
            //     ],
            // )
            // .await;

            loop {
                while let Ok(event) = event_loop.poll().await {
                    // println!("Notification = {:?}", event);
                    // match notification {
                    //     Ok(event) => {
                    match event {
                        rumqttc::Event::Incoming(incoming) => {
                            // println!("Incoming = {:?}", incoming);

                            match incoming {
                                // rumqttc::Packet::Connect(_) => todo!(),
                                // rumqttc::Packet::ConnAck(_) => todo!(),
                                rumqttc::Packet::Publish(packet) => {
                                    // println!("Publish = {:?}", packet);
                                    let topic = packet.topic;
                                    let payload = packet.payload;

                                    // runner.handle_incoming_message(&topic, payload).await;
                                }
                                // rumqttc::Packet::PubAck(_) => todo!(),
                                // rumqttc::Packet::PubRec(_) => todo!(),
                                // rumqttc::Packet::PubRel(_) => todo!(),
                                // rumqttc::Packet::PubComp(_) => todo!(),
                                // rumqttc::Packet::Subscribe(_) => todo!(),
                                // rumqttc::Packet::SubAck(_) => todo!(),
                                // rumqttc::Packet::Unsubscribe(_) => todo!(),
                                // rumqttc::Packet::UnsubAck(_) => todo!(),
                                // rumqttc::Packet::PingReq => todo!(),
                                // rumqttc::Packet::PingResp => todo!(),
                                // rumqttc::Packet::Disconnect => todo!(),
                                _ => {}
                            }
                        }
                        rumqttc::Event::Outgoing(outgoing) => {
                            // println!("Outgoing = {:?}", outgoing);
                            match outgoing {
                                // rumqttc::Outgoing::Publish(packet) => {
                                //     // println!("Publish = {:?}", packet);
                                // }
                                // rumqttc::Outgoing::Subscribe(p) => {
                                //     // println!("Subscribe = {:?}", p);
                                // }
                                // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
                                // rumqttc::Outgoing::PubAck(_) => todo!(),
                                // rumqttc::Outgoing::PubRec(_) => todo!(),
                                // rumqttc::Outgoing::PubRel(_) => todo!(),
                                // rumqttc::Outgoing::PubComp(_) => todo!(),
                                // rumqttc::Outgoing::PingReq => todo!(),
                                // rumqttc::Outgoing::PingResp => todo!(),
                                // rumqttc::Outgoing::Disconnect => todo!(),
                                // rumqttc::Outgoing::AwaitAck(_) => todo!(),
                                _ => {}
                            }
                        } // }
                          // }
                          // Err(_) => todo!(),
                    }
                }
            }
        });

        Self { mqtt_client: client }
    }

}
