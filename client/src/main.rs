pub mod config;
pub mod scenes;
pub mod ui;
pub mod map_data;
pub mod asset_updater;

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use common::{ClientState, UpdateEvent};
use tracing::{info, error};

use macroquad::{prelude::*, ui::{root_ui, Skin}};

use directories::BaseDirs;
use tungstenite::{connect, Message::Text};
use url::Url;

use crate::{
    config::{load_servers, save_servers}, 
    scenes::{inside::render_inside, login::render_login, message_popup::show_popup, outside::render_outside, server_select::run_server_selector}, ui::theme::generate_theme
};

static TIMEOUT: f64 = 3.;

#[macroquad::main("Gwynedd Valley")]
async fn main() {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);

    // Generate Config Directories
    let mut config_path = String::from("");
    if let Some(base_dirs) = BaseDirs::new() {
        config_path = base_dirs.config_dir().to_str().unwrap().to_string();
        config_path.push_str("/gwynedd-valley/")
    } else {
        config_path.push_str("gwynedd-valley/");
    }

    let mut asset_path = String::from("");
    if let Some(base_dirs) = BaseDirs::new() {
        asset_path = base_dirs.data_dir().to_str().unwrap().to_string();
        asset_path.push_str("/gwynedd-valley/")
    } else {
        asset_path.push_str("gwynedd-valley/");
    }

    // Setup Theming
    let _default_theme = Arc::new(root_ui().default_skin().clone());
    let custom_theme = Arc::new(generate_theme());

    // Update Assets
    info!("Updating assets at path: {}", &asset_path);
    match asset_updater::update_assets(&asset_path) {
        Ok(_) => (),
        Err(e) => panic!("Error updating assets: {}", e),
    }
    info!("Loading config from path: {}", &config_path);

    // Load Saved Servers
    info!("Loading saved servers...");
    let mut servers = load_servers(&config_path).unwrap_or_default();
    info!("Loading map data...");
    let map_data = map_data::import_data(&asset_path).await;

    let mut socket;
    let mut state: ClientState;

    'server_select: loop {
        // Show Server Selection Screen
        info!("Displaying server selector...");
        let server_count = servers.len();
        let server = run_server_selector(custom_theme.clone(), &mut servers).await;
        if servers.len() != server_count {
            let e = save_servers(&servers, &config_path);
            if e.is_err() {
                error!("Error saving server config:\n{}", e.unwrap_err());
            }
        }
        let mut secure = false;
        let socket2;
        'server_connect: loop {
            let server = format!("{}://{}", if secure {"wss"} else {"ws"}, &server);
            info!("Connecting to: {}", server);

            // Show connecting screen
            let mut counter = 0;
            loop {
                clear_background(GRAY);
                show_popup(&custom_theme, String::from("Connecting..."));
                if counter < 3 {
                    counter += 1;
                } else {
                    break;
                }
                next_frame().await
            }

            // Connect to Server
            info!("Creating server connection");
            let server_connection = 
                connect(Url::parse(&server).unwrap());

            if server_connection.is_err() {
                if !secure {
                    secure = true;
                    continue 'server_connect;
                } else {
                    error!("{}", server_connection.unwrap_err());
                    err_msg(&custom_theme, "!!!COULD NOT CONNECT TO SERVER!!!").await;
                    continue 'server_select;
                }
            }
            let (soc, _) = server_connection.unwrap();
            info!("Connected");
            socket2 = Some(soc);
            break 'server_connect;
        }
        if socket2.is_none() {
            continue 'server_select;
        }
        socket = socket2.unwrap();

        let auth = render_login(&custom_theme).await;
        // Show loading screen
        let mut counter = 0;
        loop {
            clear_background(GRAY);
            show_popup(&custom_theme, String::from("Loading..."));
            if counter < 3 {
                counter += 1;
            } else {
                break;
            }
            next_frame().await
        }

        info!("Logging in as '{}'", auth.username);
        let auth_ser = serde_json::to_string(&auth).expect("Failed to serialize the auth packet");
        drop(auth);
        let auth_send_status = socket.write(Text(auth_ser));
        let _ = socket.flush();
        if auth_send_status.is_err() {
            error!("Couldn't send auth packet");
            let timer = get_time();
            loop {
                if get_time() - timer > TIMEOUT {
                    continue 'server_select;
                }
                clear_background(RED);
                show_popup(&custom_theme, String::from("!!!Couldn't send auth packet!!!"));
                next_frame().await
            }
        }

        info!("Getting Client State");
        let server_msg = socket.read();
        if server_msg.is_err() {
            error!("{}", server_msg.unwrap_err());
            err_msg(&custom_theme, "Server connection closed").await;
            continue 'server_select;
        }
        let server_msg = server_msg.unwrap();
        let mut state2 = None;

        if server_msg.is_text() || server_msg.is_binary() {
            let msg = server_msg.into_text().unwrap();
            let state_temp = serde_json::from_str::<ClientState>(&msg);
            if state_temp.is_ok() {
                state2 = Some(state_temp.unwrap());
            } else {
                error!("Error parsing server message: {}", state_temp.unwrap_err());
            }
        }
        if state2.is_none() {
            let timer = get_time();
            loop {
                if get_time() - timer > TIMEOUT {
                    continue 'server_select;
                }
                clear_background(RED);
                show_popup(&custom_theme, String::from("!!!Server did not send state!!!"));
                next_frame().await
            }
        }
        state = state2.unwrap();
        if !state.authenticated {
            let _ = socket.close(None);
            let timer = get_time();
            loop {
                if get_time() - timer > TIMEOUT {
                    continue 'server_select;
                }
                clear_background(RED);
                show_popup(&custom_theme, String::from("Authentication Error"));
                next_frame().await
            }
        }
        break;
    }

    let update_queue: Arc<Mutex<VecDeque<UpdateEvent>>> = Arc::new(Mutex::new(VecDeque::new()));
    let send_queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));

    let thr_update_queue = update_queue.clone();
    let thr_send_queue = send_queue.clone();
    // let soc = &socket;
    let handle = thread::spawn(move || {
        loop {
            // Get Updates
            let msg = socket.read();
            if msg.is_err() {
                info!("Socket Error");
                return;
            }
            let msg = msg.unwrap();
            if msg.is_text() || msg.is_binary() {
                let text = msg.into_text().unwrap();
                let new_state = serde_json::from_str::<UpdateEvent>(&text).unwrap();
                thr_update_queue.lock().unwrap().push_back(new_state);
            } else if msg.is_ping() {
                let ping = msg.into_data();
                info!("Ping");
                let _ = socket.send(tungstenite::Message::Pong(ping));
                let _ = socket.flush();
            }
            // Send Updates
            let mut send_lock = thr_send_queue.lock().unwrap();
            while let Some(update) = send_lock.pop_front() {
                info!("Update Sent: {}", &update);
                let _ = socket.send(Text(update));
                let _ = socket.flush();
            }
        }
    });

    let nav = render_outside(&custom_theme, &asset_path, &map_data.outside, &mut state, send_queue.clone()).await;
    info!("Nav: {}", &nav);
    render_inside(&custom_theme, &asset_path, &map_data.insides.first().unwrap(), &mut state, update_queue.clone(), send_queue.clone()).await;

    handle.join().unwrap();
}

async fn err_msg(custom_theme: &Skin, msg: &str) {
    let timer = get_time();
    loop {
        if get_time() - timer > TIMEOUT {
            break;
        }
        clear_background(RED);
        show_popup(&custom_theme, String::from(msg));
        next_frame().await
    }
}