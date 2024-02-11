use std::sync::Arc;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};

pub async fn run_server_selector(theme: Arc<Skin>, saved_servers: Arc<Vec<String>>) -> String {
    let servers = saved_servers.clone();
    loop {
        let mut connect = String::new();
        root_ui().push_skin(&theme);
        clear_background(GRAY);
        widgets::Window::new(
            hash!(),
            Vec2::new(screen_width() / 10.0, screen_height() / 10.0),
            Vec2::new(screen_width() / 10.0 * 8.0, screen_height() / 10.0 * 8.0),
        )
        .label("Server Select")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            let label_size = ui.calc_size("Select Server");
            ui.label(
                Vec2::new(
                    (screen_width() / 10.0 * 8.0) / 2.0 - (label_size.x / 2.0),
                    0.0,
                ),
                "Select Server",
            );

            for server in servers.to_vec() {
                let server = server.as_str();
                ui.group(
                    hash!("servers", server),
                    Vec2::new(screen_width() / 10.0 * 8.0, 50.0),
                    |ui| {
                        ui.label(Vec2::new(0.0, 25./2.), server);
                        if ui.button(
                            Vec2::new((screen_width() / 10.0 * 8.0) - 200.0, 4.0),
                            "Connect",
                        ) {
                            connect.push_str(server);
                        };
                    },
                );
            }
            ui.button(
                Vec2::new(
                    (screen_width() / 10. * 8. / 2.) - (143.04199 / 2.),
                    screen_height() / 10.0 * 8.0 - 100.,
                ),
                "Add Server",
            );
        });
        if connect.len() > 0 {
            return connect;
        }
        root_ui().pop_skin();
        next_frame().await
    }
}
