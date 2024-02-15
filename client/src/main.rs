pub mod config;
pub mod scenes;
pub mod ui;

use std::sync::Arc;

use common::ClientState;
use git2::{build::CheckoutBuilder, Repository};
use tracing::{info, error};

use macroquad::{prelude::*, ui::{root_ui, Skin}};

use directories::BaseDirs;
use tungstenite::{connect, http::StatusCode, Message::Text};
use url::Url;

use crate::{
    config::{load_servers, save_servers}, 
    scenes::{login::render_login, message_popup::show_popup, outside::render_outside, server_select::run_server_selector}, ui::theme::generate_theme
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
    let url = "https://github.com/DarkCoder28/CIS4000_Capstone-Spring2024-ASSETS.git";
    let path = std::path::Path::new(&asset_path);
    if std::path::Path::exists(&path) {
        let repo = match Repository::open(&asset_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
        match repo.checkout_head(Some(CheckoutBuilder::new().force().into())) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
                loop {
                    clear_background(GRAY);
                    show_popup(&custom_theme, String::from("Asset Update Failed!"));
                    next_frame().await
                }
            }
        }
    } else {
        let repo = Repository::clone(url, &asset_path);
        if repo.is_err() {
            panic!("Failed to download assets: {}", repo.err().unwrap().message());
        }
    }
    info!("Loading config from path: {}", &config_path);

    // Load Saved Servers
    info!("Loading saved servers...");
    let mut servers = load_servers(&config_path).unwrap_or_default();

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
        let socket;
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
            // let (mut write, mut read) = socket.split();
            info!("Connected");
            socket = Some(soc);
            break 'server_connect;
        }
        if socket.is_none() {
            continue 'server_select;
        }
        let mut socket = socket.unwrap();

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
        let mut state = None;

        if server_msg.is_text() || server_msg.is_binary() {
            let msg = server_msg.into_text().unwrap();
            let state_temp = serde_json::from_str::<ClientState>(&msg);
            if state_temp.is_ok() {
                state = Some(state_temp.unwrap());
            } else {
                error!("Error parsing server message: {}", state_temp.unwrap_err());
            }
        }
        if state.is_none() {
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
        let state = state.unwrap();
        if !state.authenticated {
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

    let nav = render_outside(&custom_theme, &asset_path).await;
    info!("Nav: {}", &nav);

    loop {
        clear_background(PURPLE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
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