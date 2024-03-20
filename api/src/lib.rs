use rocket::{
    futures::{FutureExt, SinkExt, StreamExt},
    get,
    http::Status,
};
use shared::{
    request::{Request, RequestEvent},
    response::{ErrorRes, RealtimeResponse, ResponseEvents},
};
use ws::{Channel, Message, WebSocket};

#[get("/game/<id>")]
pub fn alternative_game_stream(ws: WebSocket, id: &str) -> Channel<'static> {
    let game_id = id.to_string().parse::<i32>();
    ws.channel(move |mut stream| {
        let game_id = match game_id {
            Ok(id) => id,
            Err(_) => {
                return stream.close(None).boxed();
            }
        };

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
                    let err = RealtimeResponse::<ErrorRes> {
                        event: ResponseEvents::Error.to_string(),
                        data: None,
                        error: Some(String::from("Invalid message type")),
                        error_code: Some(1),
                    };

                    stream
                        .send(Message::Text(serde_json::to_string(&err).unwrap()))
                        .await
                        .unwrap();
                }

                let request_obj =
                    serde_json::from_str::<Request>(message.to_string().as_str()).unwrap();
                let event_type = match RequestEvent::from_string(request_obj.event) {
                    Ok(event) => event,
                    Err(e) => {
                        let err = RealtimeResponse::<ErrorRes> {
                            event: ResponseEvents::Error.to_string(),
                            data: None,
                            error: Some(e.message),
                            error_code: Some(Status::BadRequest.code as i32),
                        };

                        stream
                            .send(Message::Text(serde_json::to_string(&err).unwrap()))
                            .await
                            .unwrap();
                        continue;
                    }
                };

                match event_type {
                    RequestEvent::JoinGame => {
                        match service::player_service::join_game(game_id, player_id) {
                            Ok(_) => (),
                            Err(e) => {
                                let err = RealtimeResponse::<ErrorRes> {
                                    event: ResponseEvents::Error.to_string(),
                                    data: None,
                                    error: Some(e.message),
                                    error_code: Some(Status::BadRequest.code as i32),
                                };

                                stream
                                    .send(Message::Text(serde_json::to_string(&err).unwrap()))
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                    RequestEvent::LeaveGame => {}
                    RequestEvent::StartGame => {}
                    RequestEvent::FinishRound => {}
                    RequestEvent::StartNextRound => {}
                    RequestEvent::FinishGame => {}
                    RequestEvent::SendRoundResult => {}
                }
            }
            Ok(())
        })
    })
}
