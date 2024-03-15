use std::{borrow::Borrow, sync::Mutex};

use crate::{
    error::ErrorCodes,
    events::EventType,
    repository::{
        GameRepository, PlayerRoundScoringRepository, PlayerScoringRepository, RoundRepository,
    },
};
use rocket::{
    futures::{SinkExt, StreamExt},
    State,
};
use ws::{Channel, Message, WebSocket};

use super::{RequestMessage, ResponseMessage};

#[get("/game/<id>")]
pub fn alternative_game_stream(
    ws: WebSocket,
    id: &str,
    game_repo: &State<Mutex<GameRepository>>,
    player_round_scoring_repo: &State<Mutex<PlayerRoundScoringRepository>>,
    round_repo: &State<Mutex<RoundRepository>>,
    player_scoring_repo: &State<Mutex<PlayerScoringRepository>>,
) -> Channel<'static> {
    let game_id = id.to_string();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(msg) = stream.next().await {
                let message = msg.unwrap();
                println!(
                    "Received message: {}, game id: {}",
                    message.clone(),
                    game_id
                );
                if message.is_empty() {
                    continue;
                }

                if !message.is_text() {
                    let err = ResponseMessage {
                        event_type: "error".to_string(),
                        data: None,
                        error: Some("Invalid message type".to_string()),
                        error_code: Some(ErrorCodes::InvalidMessageType as i32),
                    };

                    let err_msg = Message::from(serde_json::to_string(err.borrow()).unwrap());

                    stream.send(err_msg).await.unwrap();
                }

                let request_obj =
                    serde_json::from_str::<RequestMessage>(message.to_string().as_str()).unwrap();
                let event_type = match EventType::from_string(request_obj.event_type) {
                    Ok(event) => event,
                    Err(err) => {
                        let err = ResponseMessage {
                            event_type: "error".to_string(),
                            data: None,
                            error: Some(err.message),
                            error_code: Some(err.code as i32),
                        };

                        let err_msg = Message::from(serde_json::to_string(err.borrow()).unwrap());

                        stream.send(err_msg).await.unwrap();
                        continue;
                    }
                };

                // TODO handle event by type
                // TODO subscribe to redis channel for game events
            }
            Ok(())
        })
    })
}
