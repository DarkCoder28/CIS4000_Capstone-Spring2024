pub mod client_auth;
pub mod handle_client;

use crate::handle_client::handle_client;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::net::TcpListener;
use std::sync::Arc;
use tracing::error;

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
    // let (master_broadcast, watch) = broadcast::channel::<(u8, UpdateEvent)>(512);
    // let _watcher = watcher(watch);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                // let mb = master_broadcast.clone();
                tokio::spawn(async move {
                    let stream = acceptor.accept(stream).unwrap();
                    let mut peer_id = [0u8; 1];
                    let _ = openssl::rand::rand_bytes(&mut peer_id);
                    let peer_id = peer_id[0];
                    handle_client(peer_id, stream).await;
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}

// async fn watcher(mut master_broadcast: Receiver<(u8, UpdateEvent)>) {
//     loop {
//         if let Ok((peer, msg)) = master_broadcast.blocking_recv() {
//             info!("Event: {}: {:#?}", peer, msg);
//         } else {
//             error!("Issue with master broadcast");
//         }
//     }
// }
