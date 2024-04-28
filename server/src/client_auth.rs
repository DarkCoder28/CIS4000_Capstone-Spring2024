use std::{net::TcpStream, sync::{Arc, Mutex}};

use common::{conn_lib::read_stream, ClientAuth, ClientState};
use openssl::ssl::SslStream;
use tracing::{error, info};

use crate::mongo;



pub async fn auth(stream: Arc<Mutex<SslStream<TcpStream>>>, peer: u8) -> Option<ClientState> {
    // Receive Client Auth Message
    info!("Client '{}': Waiting for client auth", peer);
    // let mut auth_msg = String::new();
    // let res = read.lock().expect("Couldn't Lock").read_to_string(&mut auth_msg);
    // let res = read.read_to_string(&mut auth_msg).await;
    // let auth_msg = conn_lib::read_msg_server(read, key).await;
    let auth_msg = read_stream(stream.clone()).await;
    if auth_msg.is_err() {
        error!("Client '{}': Client didn't send auth packet. Closing connection!", peer);
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    info!("Client '{}': {}", peer, auth_msg);
    let auth = serde_json::from_str::<ClientAuth>(&auth_msg);
    drop(auth_msg);
    if auth.is_err() {
        error!("Client '{}': Client's auth packet couldn't be deserialized: {}", peer, &auth.unwrap_err());
        return None;
    }
    let auth = auth.unwrap();
    info!("Client '{}': Received client authentication", peer);

    info!("Client '{}': Client username: {}", peer, &auth.username);

    // Get client state or generate new
    info!("Client '{}': Getting client state", peer);
    let client_state = mongo::auth_and_get_client(&auth).await;
    drop(auth);
    Some(client_state)
}