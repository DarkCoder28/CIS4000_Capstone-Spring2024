pub mod client_auth;
pub mod handle_client;
pub mod mongo;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use tokio::sync::broadcast::{self, Sender};
use tracing::{error, info};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use crate::handle_client::handle_client;

#[tokio::main]
async fn main() {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("/srv/certs/server.key.pem", SslFiletype::PEM)
        .unwrap();
    // acceptor.set_certificate_chain_file("/srv/certs/cert.pem").unwrap();
    match acceptor.set_certificate_file("/srv/certs/cert.pem", SslFiletype::PEM) {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
            return;
        }
    }
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    // Setup Master Broadcast Channel
    let (master_broadcast, _) = broadcast::channel::<String>(16);
    let _watcher = watcher(master_broadcast.clone());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let mb = master_broadcast.clone();
                tokio::spawn(async move {
                    let stream = acceptor.accept(stream).unwrap();
                    let mut peer_id = [0u8; 1];
                    let _ = openssl::rand::rand_bytes(&mut peer_id);
                    let peer_id = peer_id[0];
                    handle_client(mb, peer_id, stream).await;
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}

async fn watcher(master_broadcast: Sender<String>) {
    let mut sub = master_broadcast.subscribe();
    while let Ok(msg) = sub.recv().await {
        info!("Event: {}", msg);
    }
}