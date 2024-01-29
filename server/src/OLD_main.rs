use std::{borrow::Cow, env};
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
use tracing::info;
use tokio::{io::Result, net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender}};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};

#[tokio::main]
pub async fn main() -> Result<()> {
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_subscriber);

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind port");
    info!("Listening on: {}", addr);

    let (tx, mut rx) = broadcast::channel::<String>(16);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(accept_connection(stream, tx));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, tx: Sender<String>) {
    let addr = stream.peer_addr().expect("Connected streams should have a peer address");
    info!("Client connected at: {}", &addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the WebSocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();
    // Client initiates communication; sends user session
    let client_session = match read.try_filter(|msg| future::ready(msg.is_text())).next().await {
        Some(res) => {
            match res {
                Ok(msg) => {
                    String::from(msg.to_text().expect("This should be text"))
                },
                Err(_) => {
                    let _ = write.close().await;
                    return;
                }
            }
        },
        None => {
            let _ = write.close().await;
            return;
        }
    };
    let tx_forwarder = tx.clone();
    

    // Send maps and client data
}