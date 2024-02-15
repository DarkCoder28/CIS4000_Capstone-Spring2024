use common::{ClientAuth, ClientState};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender}};
use tracing::{error, info, instrument};
use std::net::SocketAddr;
use tokio_tungstenite::{accept_async, tungstenite::{Error, Message::{self, Text}, Result}, WebSocketStream};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Setup Logging
//     let tracing_sub = tracing_subscriber::FmtSubscriber::new();
//     let _ = tracing::subscriber::set_global_default(tracing_sub);
//     // Setup Master Broadcast
//     let (master_broadcast, _) = broadcast::channel::<String>(16);

//     // Create the event loop and TCP listener we'll accept connections on.
//     let try_socket = TcpListener::bind("0.0.0.0:3000").await;
//     let listener = try_socket.expect("Failed to bind");
//     tracing::info!("Listening on 0.0.0.0:3000");

//     while let Ok((stream, _)) = listener.accept().await {
//         tokio::spawn(accept_connection(stream, , master_broadcast.clone()));
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);
    // Setup Master Broadcast
    let (master_broadcast, _) = broadcast::channel::<String>(16);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, master_broadcast.clone()));
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, master_broadcast: Sender<String>) {
    if let Err(e) = handle_connection(peer, stream, master_broadcast).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

// #[instrument(name="client")]
async fn handle_connection(peer: SocketAddr, stream: TcpStream, master_broadcast: Sender<String>) -> Result<()> {
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

    let f = client_async(&mut ws_write, master_broadcast.clone());

    while let Some(msg) = ws_read.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            let _ = master_broadcast.send(msg.into_text().unwrap());
        }
    }

    f.await;

    Ok(())
}

async fn client_async(ws: &mut SplitSink<WebSocketStream<TcpStream>, Message>, master_broadcast: Sender<String>) {
    //
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
    let mut client_state = ClientState::new(&auth.username);
    drop(auth);
    client_state.authenticated = true;
    Some(client_state)
}