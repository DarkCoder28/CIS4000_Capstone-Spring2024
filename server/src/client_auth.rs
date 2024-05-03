use std::{
    net::TcpStream,
    path::Path,
    sync::{Arc, Mutex},
};

use common::{conn_lib::read_stream, ClientAuth, UserStore};
use openssl::ssl::SslStream;
use tokio::fs;
use tracing::{error, info};

pub async fn auth(stream: Arc<Mutex<SslStream<TcpStream>>>, peer: u8) -> Option<UserStore> {
    // Receive Client Auth Message
    info!("Client '{}': Waiting for client auth", peer);
    // let mut auth_msg = String::new();
    // let res = read.lock().expect("Couldn't Lock").read_to_string(&mut auth_msg);
    // let res = read.read_to_string(&mut auth_msg).await;
    // let auth_msg = conn_lib::read_msg_server(read, key).await;
    let auth_msg = read_stream(stream.clone()).await;
    if auth_msg.is_err() {
        error!(
            "Client '{}': Client didn't send auth packet. Closing connection!",
            peer
        );
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    info!("Client '{}': {}", peer, auth_msg);
    let auth = serde_json::from_str::<ClientAuth>(&auth_msg);
    drop(auth_msg);
    if auth.is_err() {
        error!(
            "Client '{}': Client's auth packet couldn't be deserialized: {}",
            peer,
            &auth.unwrap_err()
        );
        return None;
    }
    let auth = auth.unwrap();
    info!("Client '{}': Received client authentication", peer);
    info!("Client '{}': Client username: {}", peer, &auth.username);
    info!("Client '{}': Getting user store", peer);
    let user_data = get_user_file(auth.username.clone()).await;
    if user_data.is_some() {
        let user = user_data.unwrap();
        if user.pass_hash == auth.pass_hash {
            return Some(user);
        } else {
            let mut auth_err = UserStore::new(&auth.username, auth.pass_hash);
            auth_err.state.authenticated = false;
            return Some(auth_err);
        }
    }
    info!("Client '{}': User doesn't exist; creating new!", peer);
    let new_user = UserStore::new(&auth.username, auth.pass_hash);
    drop(auth);
    Some(new_user)
}

async fn get_user_file(username: String) -> Option<UserStore> {
    let mut file_path = String::from("/mnt/gv-data/");
    file_path.push_str(&username);
    file_path.push_str(".gvdata");
    let file_path = Path::new(&file_path);
    if file_path.try_exists().is_ok() && file_path.try_exists().unwrap() {
        let file_data = fs::read_to_string(file_path).await;
        if file_data.is_ok() {
            return Some(serde_json::from_str::<UserStore>(&file_data.unwrap()).unwrap());
        }
    }
    None
}
