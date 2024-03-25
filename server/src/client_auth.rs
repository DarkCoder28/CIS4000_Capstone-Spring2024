use std::{net::TcpStream, sync::{Arc, Mutex}};

use common::{conn_lib::read_stream, ClientAuth, ClientState};
use openssl::ssl::SslStream;
use tracing::{error, info};

use crate::mongo;



pub async fn auth(stream: Arc<Mutex<SslStream<TcpStream>>>) -> Option<ClientState> {
    // Receive Client Auth Message
    info!("Waiting for client auth");
    // let mut auth_msg = String::new();
    // let res = read.lock().expect("Couldn't Lock").read_to_string(&mut auth_msg);
    // let res = read.read_to_string(&mut auth_msg).await;
    // let auth_msg = conn_lib::read_msg_server(read, key).await;
    let auth_msg = read_stream(stream.clone()).await;
    if auth_msg.is_err() {
        error!("Client didn't send auth packet. Closing connection!");
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    info!("{}", auth_msg);
    let auth = serde_json::from_str::<ClientAuth>(&auth_msg);
    drop(auth_msg);
    if auth.is_err() {
        error!("Client's auth packet couldn't be deserialized: {}", &auth.unwrap_err());
        return None;
    }
    let auth = auth.unwrap();
    info!("Received client authentication");

    info!("Client username: {}", &auth.username);

    // Get client state or generate new
    info!("Getting client state");
    let client_state = mongo::auth_and_get_client(&auth).await;
    drop(auth);
    Some(client_state)
}