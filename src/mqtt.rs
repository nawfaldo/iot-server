use rumqttc::{AsyncClient, Event, MqttOptions, QoS, TlsConfiguration, Transport};

use std::{fs, time::Duration};
use tokio::task;

pub async fn start_mqtt_client() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("MQTT_HOST").unwrap();
    let port: u16 = std::env::var("MQTT_PORT")
        .expect("MQTT_PORT must be set in .env")
        .parse()
        .expect("MQTT_PORT must be a valid u16 integer");
        
        let client = std::env::var("MQTT_CLIENT").unwrap();

    let mut mqttoptions = MqttOptions::new(client, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let username = std::env::var("MQTT_USERNAME").unwrap();
    let password = std::env::var("MQTT_PASSWORD").unwrap();

    mqttoptions.set_credentials(username, password);

    let ca_path = std::env::var("MQTT_CA_PATH").unwrap();

    let ca = fs::read(ca_path)?;

    let tls = TlsConfiguration::Simple {
        ca: ca,
        alpn: None,
        client_auth: None,
    };
    mqttoptions.set_transport(Transport::tls_with_config(tls.into()));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    task::spawn(async move {
        println!("MQTT client event loop started.");

        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(rumqttc::Packet::Publish(p))) => {
                    println!("Received message on topic: {}", p.topic);
                    println!("Payload: {:?}", String::from_utf8_lossy(&p.payload));
                }
                Ok(Event::Outgoing(rumqttc::Outgoing::PingReq)) => {}
                Ok(event) => {}
                Err(e) => {
                    println!("MQTT Error: {:?}", e);
                }
            }
        }
    });

    let subscribe_client = client.clone();
    task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await; // Give some time for connection
        match subscribe_client.subscribe("test", QoS::AtLeastOnce).await {
            Ok(_) => println!("Subscribed to all desired topics"),
            Err(e) => println!("Failed to subscribe: {:?}", e),
        }
    });

    Ok(())
}

pub async fn publish_message(
    client: &AsyncClient,
    topic: &str,
    payload: &str,
) -> Result<(), rumqttc::ClientError> {
    client
        .publish(topic, QoS::AtMostOnce, false, payload.as_bytes())
        .await
}
