use std::error::Error;

use redis::{Client, Commands};

use super::Message;

pub fn publish_message(redis_client: Client, message: Message) -> Result<(), Box<dyn Error>> {
    let mut connection = redis_client.get_connection()?;
    let message_json = serde_json::to_string(&message)?;

    connection.publish(message.channel, message_json)?;

    Ok(())
}
