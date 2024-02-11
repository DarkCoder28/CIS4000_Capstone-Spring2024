use std::sync::Arc;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};


pub async fn run_server_selector(theme: Arc<Skin>, servers: &mut Vec<String>) -> String {
    let mut show_add_server = false;
    let mut server_to_add = String::new();
    let mut to_delete: (usize, String) = (usize::MAX, String::new());
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

            for (i, server) in servers.to_vec().iter().enumerate() {
                let server = server.as_str();
                ui.group(
                    hash!("servers", server),
                    Vec2::new(screen_width() / 10.0 * 8.0, 50.0),
                    |ui| {
                        ui.label(Vec2::new(0.0, 25. / 2.), server);
                        if ui.button(
                            Vec2::new((screen_width()/10.*8.)-300., 4.), 
                            "Delete"
                        ) {
                            to_delete = (i, server.to_string());
                        }
                        if ui.button(
                            Vec2::new((screen_width() / 10.0 * 8.0) - 200.0, 4.0),
                            "Connect",
                        ) {
                            connect.push_str(server);
                        };
                    },
                );
            }
            if ui.button(
                Vec2::new(
                    (screen_width() / 10. * 8. / 2.) - (143.04199 / 2.),
                    screen_height() / 10.0 * 8.0 - 100.,
                ),
                "Add Server",
            ) {
                show_add_server = true;
            }
        });
        if show_add_server {
            widgets::Window::new(
                hash!(),
                Vec2::new(screen_width() / 2. - 200., screen_height() / 2. - 75.),
                Vec2::new(400., 150.),
            )
            .label("Add Server")
            .titlebar(false)
            .ui(&mut root_ui(), |ui| {
                let label_size = ui.calc_size("Add Server");
                ui.label(None, "");
                ui.label(Vec2::new(200. - (label_size.x / 2.), 0.), "Add Server");
                widgets::InputText::new(hash!())
                    .size(Vec2::new(350., 35.))
                    .ui(ui, &mut server_to_add);
                // ui.input_text(hash!(), "", &mut server_to_add);
                if ui.button(Vec2::new(125., 75.), "Submit") {
                    servers.push(server_to_add.clone());
                    server_to_add = String::new();
                    show_add_server = false;
                }
            });
        }
        if to_delete.0 != usize::MAX {
            widgets::Window::new(
                hash!(),
                Vec2::new(screen_width() / 2. - 200., screen_height() / 2. - 75.),
                Vec2::new(400., 150.),
            )
            .label("Delete Server")
            .titlebar(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(Vec2::new(87.5, 0.), "Delete Server");
                let label_size = ui.calc_size(&to_delete.1);
                ui.label(Vec2::new(200.-(label_size.x/2.)-15., 35.), &to_delete.1);
                if ui.button(Vec2::new(55., 75.), "Cancel") {
                    to_delete = (usize::MAX, String::new());
                }
                if ui.button(Vec2::new(195., 75.), "Submit") {
                    servers.remove(to_delete.0);
                    to_delete = (usize::MAX, String::new());
                }
            });
        }
        if connect.len() > 0 {
            return connect;
        }
        root_ui().pop_skin();
        next_frame().await
    }
}
