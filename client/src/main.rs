pub mod config;
pub mod scenes;

use std::sync::Arc;

use tracing::{info, error};

use macroquad::prelude::*;

use directories::BaseDirs;
use tungstenite::connect;
use url::Url;

use crate::{
    config::load_servers, 
    scenes::server_select::run_server_selector
};

#[macroquad::main("Capstone Game")]
async fn main() {
    // Setup Logging
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);

    // Generate Config Directory
    let mut config_path = String::from("");
    if let Some(base_dirs) = BaseDirs::new() {
        config_path = base_dirs.config_dir().to_str().unwrap().to_string();
        config_path.push_str("/capstone-game/")
    } else {
        config_path.push_str("capstone-game/");
    }

    info!("Loading config from path: {}", &config_path);

    // Load Saved Servers
    info!("Loading saved servers...");
    let servers = Arc::new(load_servers(&config_path).unwrap_or_default());

    // Show Server Selection Screen
    info!("Displaying server selector...");
    let server = run_server_selector(servers).await;

    // Show connecting screen
    clear_background(GRAY);
    draw_text("Connecting...", screen_width()/2.0, screen_height()/2.0, 32f32, BLUE);

    // Connect to Server
    info!("Creating server connection");
    let server_connection = 
        connect(Url::parse(&std::env::var("SERVER")
            .expect("SERVER env var must be defined.")).unwrap());
    if server_connection.is_err() {
        error!("{}", server_connection.unwrap_err());
        loop {
            clear_background(RED);
            let message = "!!!COULD NOT CONNECT TO SERVER!!!";
            let center = get_text_center(message, None, 32, 1.0, 0.0);
            draw_text(&message, (screen_width()/2.0)-center.x, (screen_height()/2.0)-center.y, 32.0, BLUE);
            next_frame().await
        }
    }
    let (mut _socket, _response) = server_connection.unwrap();

    loop {
        clear_background(PURPLE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}