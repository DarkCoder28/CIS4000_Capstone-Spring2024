pub mod mongo;

use common::{ClientAuth, ClientState, UpdateEvent};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender}};
use tracing::{error, info, instrument};
use std::{net::SocketAddr, time::Duration};
use tokio_tungstenite::{accept_async, tungstenite::{Error, Message::{self, Text}, Result}, WebSocketStream};


#[tokio::main]
async fn main() {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);
    // Setup Master Broadcast
    let (master_broadcast, _) = broadcast::channel::<Message>(16);
    let p = pinger(master_broadcast.clone());
    let watcher = watcher(master_broadcast.clone());

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, master_broadcast.clone()));
    }

    p.await;
    watcher.await;
}

async fn watcher(master_broadcast: Sender<Message>) {
    let mut sub = master_broadcast.subscribe();
    while let Ok(msg) = sub.recv().await {
        match msg {
            Text(x) => {
                info!("Event: {}", x);
            },
            Message::Ping(_) => {
                info!("Ping");
            },
            _ => {}
        }
        // if msg.is_binary() || msg.is_text() {
            
        // }
    }
}

async fn pinger(master_broadcast: Sender<Message>) {
    loop {
        let rand_data = vec![0b10101010, 64];
        let _ = master_broadcast.send(Message::Ping(rand_data));
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, master_broadcast: Sender<Message>) {
    if let Err(e) = handle_connection(peer, stream, master_broadcast).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

#[instrument(name="client")]
async fn handle_connection(peer: SocketAddr, stream: TcpStream, master_broadcast: Sender<Message>) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut ws_write, mut ws_read) = ws_stream.split();

    info!("New WebSocket connection: {}", peer);

    let client_state = auth(&mut ws_read).await;
    if client_state.is_none() {
        return Ok(());
    }
    let mut client_state = client_state.unwrap();
    info!("Sending client state");
    let init_state = serde_json::to_string(&client_state).unwrap();
    let _ = ws_write.send(Text(init_state)).await;
    info!("Client state sent");

    let f = client_async(peer.to_string(), &mut ws_write, master_broadcast.clone());

    while let Some(msg) = ws_read.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            let update = msg.clone().into_text().unwrap();
            let update = serde_json::from_str::<UpdateEvent>(&update).unwrap();
            client_state.apply_update(&update);
            let _ = master_broadcast.send(msg);
            if update.logout {
                break;
            }
        }
    }

    let client_exit_state = serde_json::to_string(&client_state).unwrap();
    info!("Client '{}' exit state:\n{}", peer, client_exit_state);

    f.await;

    Ok(())
}

async fn client_async(client_id: String, ws: &mut SplitSink<WebSocketStream<TcpStream>, Message>, master_broadcast: Sender<Message>) {
    let mut receive = master_broadcast.subscribe();
    loop {
        let msg = receive.recv().await.unwrap();
        let status = ws.send(msg).await;
        if status.is_err() {
            info!("Client '{}' disconnected", client_id);
            return;
        }
    }
}

async fn auth(ws_read: &mut SplitStream<WebSocketStream<TcpStream>>) -> Option<ClientState> {
    // Receive Client Auth Message
    info!("Waiting for client auth");
    let auth_msg = ws_read.next().await;
    if auth_msg.is_none() {
        error!("Client didn't send auth packet. Closing connection!");
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    if auth_msg.is_err() {
        error!("Client didn't send auth packet. Closing connection!");
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    let auth_msg = auth_msg.into_text();
    if auth_msg.is_err() {
        error!("Client's auth message wasn't a string");
        return None;
    }
    let auth_msg = auth_msg.unwrap();
    let auth = serde_json::from_str::<ClientAuth>(&auth_msg);
    drop(auth_msg);
    if auth.is_err() {
        error!("Client's auth packet couldn't be deserialized");
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