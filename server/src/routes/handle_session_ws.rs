use axum::{
    response::Response,
    extract::{WebSocketUpgrade, ws::{WebSocket, Message::Text}, State}
};
// use futures_util::{SinkExt, StreamExt};
// use serde::Serialize;
use tower_cookies::Cookies;
use tracing::info;

use crate::state::AppState;

pub async fn ws_session_handler(cookies: Cookies, ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    let user = crate::auth::get_user_from_session_cookie(state.mongo.clone(), cookies.clone()).await;
    if user.is_none() {
    }
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, _state: AppState) -> () {
    let _ = socket.send(Text("Hello".to_string())).await;
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(msg) => {
                let msg = msg.to_text().unwrap_or("");
                info!(msg);
                if socket.send(Text(String::from("Echo: ") + msg)).await.is_err() {
                    // Client Disconnected
                    return;
                }
            },
            Err(_) => {
                // Client Disconnected
                return;
            }
        }
    }
    //
    // while let Some(msg) = socket.recv().await {
    //     match msg {
    //         Ok(msg) => {
    //             let msg = msg.to_text().unwrap_or("");
    //             if socket.send(Text(serialized_recipes)).await.is_err() {
    //                 // Client Disconnected
    //                 return;
    //             }
    //         },
    //         Err(_) => {
    //             // Client Disconnected
    //             return;
    //         }
    //     }
    // }
}