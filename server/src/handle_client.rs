use std::{
    collections::VecDeque, net::TcpStream, sync::{Arc, Mutex}
};

use common::{
    conn_lib::{read_stream, write_flush},
    UpdateEvent,
};
use openssl::ssl::SslStream;
use tokio::sync::broadcast::Sender;
use tracing::{error, info};

use crate::client_auth;

pub async fn handle_client(
    master_broadcast: Sender<(u8, UpdateEvent)>,
    peer: u8,
    stream: SslStream<TcpStream>,
) {
    let stream = Arc::new(Mutex::new(stream));

    info!("New Socket connection: {}", peer);

    // Receive Client Auth Packet
    let client_state = client_auth::auth(stream.clone(), peer).await;
    if client_state.is_none() {
        return;
    }
    let mut client_state = client_state.unwrap();
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

    let mut master_broadcast_reader = master_broadcast.subscribe();

    'client_loop: loop {
        // Get updates from client & send them to master_broadcast...
        if let Ok(update) = read_stream(stream.clone()).await {
            let event = serde_json::from_str::<UpdateEvent>(&update).unwrap();
            client_state.apply_update(&event);
            let logout = event.logout;
            match master_broadcast.send((peer, event)) {
                Ok(_) => (),
                Err(e) => {
                    error!("Client '{}': Error Broadcasting Update: {}", peer, e);
                }
            };
            if logout {
                break 'client_loop;
            }
        }
        // Aggregate other updates...
        let mut relevant_updates_queue = VecDeque::new();
        while let Ok((peer_id, event)) = master_broadcast_reader.recv().await {
            if peer_id != peer {
                relevant_updates_queue.push_back(event)
            }
        }
        // Send all updates to client
        while let Some(event) = relevant_updates_queue.pop_front() {
            let update = serde_json::to_string(&event).unwrap();
            info!("Client '{}': Sending \"{}\"", peer, &update);
            match write_flush(stream.clone(), update).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Client '{}': Error: {}", peer, e);
                }
            }
        }
    }

    let client_exit_state = serde_json::to_string(&client_state).unwrap();
    info!("Client '{}': exit state:\n{}", peer, client_exit_state);
}













    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // let f = client_async(peer.to_string(), stream.clone(), master_broadcast.clone());

    // loop {
    //     let msg = match read_stream(stream.clone()).await {
    //         Ok(m) => m,
    //         Err(_) => {
    //             info!("Client '{}' disconnected", peer);
    //             return;
    //         }
    //     };
    //     let update = serde_json::from_str::<UpdateEvent>(&msg).unwrap();
    //     // info!("Client '{}': {:#?}", peer, &update);
    //     client_state.apply_update(&update);
    //     let logout = update.logout;
    //     match master_broadcast.send(update) {
    //         Ok(_) => (),
    //         Err(e) => {
    //             error!("Client '{}': Master Broadcast Error: {}", peer, e);
    //         }
    //     };
    //     if logout {
    //         break;
    //     }
    // }

    // let client_exit_state = serde_json::to_string(&client_state).unwrap();
    // info!("Client '{}': exit state:\n{}", peer, client_exit_state);

    // f.await;
// }

// async fn client_async(
//     client_id: String,
//     stream: Arc<Mutex<SslStream<TcpStream>>>,
//     master_broadcast: Sender<UpdateEvent>,
// ) {
//     let mut receive = master_broadcast.subscribe();
//     loop {
//         let msg = match receive.recv().await {
//             Ok(m) => m,
//             Err(_) => {
//                 info!("Client '{}' disconnected", client_id);
//                 return;
//             }
//         };
//         let msg_ser = serde_json::to_string(&msg).unwrap();
//         let status = write_flush(stream.clone(), msg_ser).await;
//         if status.is_err() {
//             info!("Client '{}' disconnected", client_id);
//             return;
//         }
//     }
// }
