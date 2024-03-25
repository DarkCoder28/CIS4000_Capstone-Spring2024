pub mod client_auth;
pub mod mongo;

use std::net::{SocketAddr, ToSocketAddrs};
use std::io;

use common::conn_lib::SymKey;
use common::{conn_lib, UpdateEvent};
use tokio::io::{split, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Sender};
use tracing::info;

// fn load_certs() -> io::Result<Vec<CertificateDer<'static>>> {
//     certs(&mut BufReader::new(File::open(PathBuf::from("/srv/certs/cert.pem"))?)).collect()
// }

// fn load_keys() -> io::Result<PrivateKeyDer<'static>> {
//     rsa_private_keys(&mut BufReader::new(File::open(PathBuf::from("/srv/certs/server.key.rsa"))?))
//         .next()
//         .unwrap()
//         .map(Into::into)
// }

#[tokio::main]
async fn main() -> io::Result<()> {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);

    // Setup Socket
    let addr = "0.0.0.0:3000"
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
    // let certs = load_certs()?;
    // let key = load_keys()?;

    // let config = rustls::ServerConfig::builder()
    //     .with_no_client_auth()
    //     .with_single_cert(certs, key)
    //     .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    // let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    // Setup Master Broadcast Channel
    let (master_broadcast, _) = broadcast::channel::<String>(16);
    let _watcher = watcher(master_broadcast.clone());

    loop {
        info!("Waiting for client...");
        let (mut stream, peer_addr) = listener.accept().await?;
        // let acceptor = acceptor.clone();

        let mb_split = master_broadcast.clone();
        
        let fut = async move {
            // let mut stream = acceptor.accept(stream).await?;
            let conn_res = conn_lib::establish_connection_server(&mut stream).await;
            if conn_res.is_err() {
                return Err(conn_res.unwrap_err());
            }
            accept_connection(mb_split, peer_addr, &mut stream, conn_res.unwrap()).await;
            Ok(())
        };

        // let f = tokio::spawn(accept_connection(master_broadcast.clone(), peer_addr, stream, acceptor));
        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{:?}", err);
            }
        });
    }

    // _watcher.await;
}

async fn watcher(master_broadcast: Sender<String>) {
    let mut sub = master_broadcast.subscribe();
    while let Ok(msg) = sub.recv().await {
        info!("Event: {}", msg);
    }
}

async fn accept_connection(master_broadcast: Sender<String>, peer: SocketAddr, stream: &mut TcpStream, key: SymKey) {
    // let stream = stream_acceptor.accept(stream).await;
    // let stream = stream.unwrap();
    let (mut read, mut write) = split(stream);

    info!("New Socket connection: {}", peer);

    // Receive Client Auth Packet
    let client_state = client_auth::auth(&mut read, &key).await;
    if client_state.is_none() {
        return;
    }
    let mut client_state = client_state.unwrap();
    info!("Sending client state");
    let init_state = serde_json::to_string(&client_state).unwrap();
    let _ = conn_lib::send_msg_server(init_state, &mut write, &key).await;
    // let _ = write.write_all(init_state.as_bytes()).await;
    // let _ = write.flush().await;
    info!("Client state sent");

    let f = client_async(peer.to_string(), &mut write, master_broadcast.clone(), &key);

    loop {
        // let mut msg = String::new();
        // let _ = read.read_to_string(&mut msg);
        let msg = match conn_lib::read_msg_server(&mut read, &key).await {
            Ok(x) => x,
            Err(_) => {
                continue;
            }
        };
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

async fn client_async(client_id: String, write: &mut WriteHalf<&mut TcpStream>, master_broadcast: Sender<String>, key: &SymKey) {
    let mut receive = master_broadcast.subscribe();
    loop {
        let msg = receive.recv().await.unwrap();
        // let status = write.write_all(msg.as_bytes()).await;
        // let _ = write.flush().await;
        let status = conn_lib::send_msg_server(msg, write, key).await;
        if status.is_err() {
            info!("Client '{}' disconnected", client_id);
            return;
        }
    }
}