use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestMessage {
    pub event_type: String,
    pub data: Option<RequestMessageData>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestMessageData {
    // Used for round result event
    pub time_used_to_finish: Option<i32>,
    pub top_topic: Option<String>,
    pub second_topic: Option<String>,
    pub third_topic: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub event_type: String,
    pub data: Option<ResponseMessageData>,
    pub error: Option<String>,
    pub error_code: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessageData {
    pub player_id: Option<String>,
    pub topic: Option<String>,
    // TODO replace string type with final result struct
    pub final_result: Option<String>,
}
