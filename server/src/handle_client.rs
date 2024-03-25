use std::{net::TcpStream, sync::{Arc, Mutex}};

use common::{conn_lib::{write_flush, read_stream}, UpdateEvent};
use openssl::ssl::SslStream;
use tokio::sync::broadcast::Sender;
use tracing::{error, info};

use crate::client_auth;



pub async fn handle_client(master_broadcast: Sender<String>, peer: u8, stream: SslStream<TcpStream>) {
    // let stream = stream_acceptor.accept(stream).await;
    // let stream = stream.unwrap();
    // let (mut read, mut write) = split(stream);
    let stream = Arc::new(Mutex::new(stream));

    info!("New Socket connection: {}", peer);

    // Receive Client Auth Packet
    let client_state = client_auth::auth(stream.clone()).await;
    if client_state.is_none() {
        return;
    }
    let mut client_state = client_state.unwrap();
    info!("Sending client state");
    let init_state = serde_json::to_string(&client_state).unwrap();
    // let _ = conn_lib::send_msg_server(init_state, &mut write, &key).await;
    // let _ = stream.lock().expect("Couldn't lock stream").write_all(init_state.as_bytes());
    // let _ = stream.lock().expect(msg).flush().await;
    match write_flush(stream.clone(), init_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
            return;
            // return Err(ConnectionError::AsymError);
        }
    }
    info!("Client state sent");

    let f = client_async(peer.to_string(), stream.clone(), master_broadcast.clone());

    loop {
        // let mut msg = String::new();
        // let _ = read.read_to_string(&mut msg);
        // let msg = match conn_lib::read_msg_server(&mut read, &key).await {
        //     Ok(x) => x,
        //     Err(_) => {
        //         continue;
        //     }
        // };
        let msg = read_stream(stream.clone()).await.unwrap();
        let update = serde_json::from_str::<UpdateEvent>(&msg).unwrap();
        client_state.apply_update(&update);
        let _ = master_broadcast.send(msg);
        if update.logout {
            break;
        }
    }

    let client_exit_state = serde_json::to_string(&client_state).unwrap();
    info!("Client '{}' exit state:\n{}", peer, client_exit_state);

    f.await;
}

async fn client_async(client_id: String, stream: Arc<Mutex<SslStream<TcpStream>>>, master_broadcast: Sender<String>) {
    let mut receive = master_broadcast.subscribe();
    loop {
        let msg = receive.recv().await.unwrap();
        // let status = write.write_all(msg.as_bytes()).await;
        // let _ = write.flush().await;
        let status = write_flush(stream.clone(), msg).await;
        // let status = conn_lib::send_msg_server(msg, write, key).await;
        if status.is_err() {
            info!("Client '{}' disconnected", client_id);
            return;
        }
    }
}