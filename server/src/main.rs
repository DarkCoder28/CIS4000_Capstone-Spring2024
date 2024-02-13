use std::{env, io::Error};

use futures_util::{future, StreamExt, TryStreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender}};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);
    // Setup Master Broadcast
    let (master_broadcast, _) = broadcast::channel::<String>(16);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind("0.0.0.0:3000").await;
    let listener = try_socket.expect("Failed to bind");
    tracing::info!("Listening on 0.0.0.0:3000");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, master_broadcast.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, master_broadcast: Sender<String>) {
    let _master_broadcast_rx = master_broadcast.subscribe();
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("New client connected (address: {})", addr);
    let (write, read) = ws_stream.split();

    // 
    // 
    // CLIENT CONNECTION
    // 
    // 

    // Receive Client Auth Message
    // let 

    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}