use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::EventType;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub event_type: EventType,
    pub channel: String,
    pub publisher_id: String
}

impl Message {
    pub fn new(channel: String, event_type: EventType, publisher_id: String) -> Message {
        Message {
            id: Message::generate_uuid(),
            event_type,
            publisher_id,
            channel
        }
    }

    fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
}
