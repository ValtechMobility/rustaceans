use futures::stream::StreamExt;
use log::{info,error};

pub async fn subscribe(client: async_nats::Client) -> Result<(), Box<dyn std::error::Error>> {
    // Verbinde dich mit dem NATS-Server

    info!("try to connect to NATS!");

    // Abonniere ein Subject (Thema)
    let mut subscriber = match client.subscribe(std::env::var("NATS_TOPIC").unwrap()).await {
        Ok(sub) => sub,
        Err(e) => {
            error!("❌ Fehler beim Subscriben auf '{}': {}", std::env::var("NATS_TOPIC").unwrap(), e);
            return Err(e.into()); // oder `std::process::exit(1);`
        }
    };

    info!("✅ Abonniert auf '{}'...", std::env::var("NATS_TOPIC").unwrap());

    // ✨ Subscriber in eigenem Task
    tokio::spawn(async move {
        // Endlosschleife, um Nachrichten zu empfangen
        while let Some(message) = subscriber.next().await {
            let text = String::from_utf8_lossy(&message.payload);
            info!("📥 Neue Nachricht: {}", text);
        }
    });
        
    Ok(())
}

pub async fn publish(client: async_nats::Client, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("try to send to NATS!");
    // Verbinde dich mit dem NATS-Server
    let antwort = format!("Antwort: {}", msg);
    match client.publish(std::env::var("NATS_TOPIC").unwrap(), antwort.into()).await {
        Ok(sub) => sub,
        Err(e) => {
            error!("❌ Fehler beim pushen auf '{}': {}", std::env::var("NATS_TOPIC").unwrap(), e);
            return Err(e.into()); // oder `std::process::exit(1);`
        }
    };

    Ok(())
}