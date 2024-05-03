use std::{
    io,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use common::{
    conn_lib::{read_stream, write_flush},
    ClientState, UserStore,
};
use openssl::ssl::SslStream;
use tokio::fs;
use tracing::{error, info};

use crate::client_auth;

pub async fn handle_client(peer: u8, stream: SslStream<TcpStream>) {
    let stream = Arc::new(Mutex::new(stream));

    info!("New Socket connection: {}", peer);

    // Receive Client Auth Packet
    let user_store = client_auth::auth(stream.clone(), peer).await;
    if user_store.is_none() {
        return;
    }
    let mut user_store = user_store.unwrap();
    if user_store.state.authenticated == false {
        return;
    }
    let client_state = user_store.state.clone();
    info!("Client '{}': Sending client state", peer);
    let init_state = serde_json::to_string(&client_state).unwrap();
    match write_flush(stream.clone(), init_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
            return;
        }
    }
    info!("Client '{}': Client state sent", peer);
    loop {
        let closure = read_stream(stream.clone()).await;
        if closure.is_ok() {
            let closure_ser = closure.unwrap();
            let closure = serde_json::from_str::<ClientState>(&closure_ser).unwrap();
            info!("Client '{}': Exiting \"{}\"", peer, &closure_ser);
            user_store.state = closure;
            break;
        }
    }
    info!("Client '{}': Saving State...", peer);
    let res = save_user_state(&user_store).await;
    if res.is_err() {
        let mut user_data_exit = user_store.clone();
        user_data_exit.pass_hash = 0;
        error!(
            "Client '{}': State not saved... redacted exit data: {:#?}",
            peer, &user_data_exit
        );
    } else {
        info!("Client '{}': State Saved", peer);
    }
}

async fn save_user_state(user_data: &UserStore) -> io::Result<()> {
    let mut file_path = String::from("/mnt/gv-data/");
    file_path.push_str(&user_data.username);
    file_path.push_str(".gvdata");

    let ser = serde_json::to_string_pretty(user_data).unwrap();

    fs::write(file_path, ser.as_bytes()).await
}
