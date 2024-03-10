use std::error::Error;

use redis::{Client, ControlFlow, PubSubCommands};

pub fn subscribe(redis_client: Client, channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let mut connection = redis_client.get_connection().unwrap();
        let _: () = connection.subscribe(&[channel.clone()], |message| {
            let received = message.get_payload::<String>().unwrap();
            let payload: String = serde_json::from_str(&received).unwrap();
            println!("Received message on channel {}: {}", channel, payload);

            return ControlFlow::Continue;
        }).unwrap();
    });

    Ok(())
}
