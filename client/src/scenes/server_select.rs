use std::sync::Arc;

use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};

pub async fn run_server_selector(saved_servers: Arc<Vec<String>>) -> String {
    let servers = saved_servers.clone();
    loop {
        clear_background(GRAY);
        widgets::Window::new(
            hash!(), 
            Vec2 { x: screen_width()/10.0, y: screen_height()/10.0 }, 
            Vec2 { x: screen_width()/10.0*8.0, y: screen_height()/10.0*8.0 }
        )
            .label("Server Select")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| {
                for server in servers.to_vec() {
                    widgets::Group::new(hash!("servers", server), Vec2::new( screen_width()/10.0*7.0, screen_height()/10.0))
                        .ui(ui, |ui| {
                            //
                        });
                }
            });
        if saved_servers.len() > 5 {
            break;
        }
        next_frame().await
    }
    String::new()
}