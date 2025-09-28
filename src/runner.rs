use std::time::Duration;

use rand::{distributions::Alphanumeric, Rng};
use rumqttc::{AsyncClient, MqttOptions};

use crate::drivers::PowerSupplyDriver;

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

pub struct Runner {
    name: String,
    driver: Box<dyn PowerSupplyDriver>,
}

impl Runner {
    pub fn new(name: String, driver: Box<dyn PowerSupplyDriver>) -> Self {
        Self { name, driver }
    }

    pub fn start(&self) {
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            "localhost",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, mut event_loop) = AsyncClient::new(mqttoptions, 100);

        // Start the runner

        tokio::spawn(async move {
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
                                    // let payload = packet.payload;
                                    // let payload_str = std::str::from_utf8(&payload).unwrap();
                                    // println!("Received = {:?} {:?}", payload_str, packet.topic);

                                    // self.message_dispatcher
                                    //     .lock()
                                    //     .await
                                    //     .trigger_on_change(&packet.topic, &packet.payload)
                                    //     .await;
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

        println!("MESSAGE ENGINE STOP !! ");

        // self.driver.enable_output()
    }
}
