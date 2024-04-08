// let mut sha3_256hasher = Sha3_256Hasher::default();
// sha3_256hasher.write(b"your string here");

// let u64result = sha3_256hasher.finish();
// let bytes_result = HasherContext::finish(&mut sha3_256hasher);

use std::hash::{Hash, Hasher};

use common::ClientAuth;
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
use rs_sha3_256::Sha3_256Hasher;

pub async fn render_login(theme: &Skin) -> ClientAuth {
    let mut sha3_256hasher = Sha3_256Hasher::default();

    let mut pwd = String::new();
    let mut auth = ClientAuth {
        username: String::new(),
        pass_hash: u64::MAX,
    };

    loop {
        root_ui().push_skin(&theme);

        widgets::Window::new(
            hash!(),
            Vec2::new(screen_width() / 2. - 200., screen_height() / 2. - 75.),
            Vec2::new(400., 200.),
        )
        .label("Login")
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            let label_size = ui.calc_size("Login");
            ui.label(None, "");
            ui.label(Vec2::new(200. - (label_size.x / 2.), 0.), "Login");
            widgets::InputText::new(hash!())
                .size(Vec2::new(350., 35.))
                .ui(ui, &mut auth.username);
            widgets::InputText::new(hash!())
                .size(Vec2::new(350., 35.))
                .password(true)
                .ui(ui, &mut pwd);
            // ui.input_text(hash!(), "", &mut server_to_add);
            if ui.button(Vec2::new(125., 110.), "Submit") {
                pwd.hash(&mut sha3_256hasher);
                auth.pass_hash = sha3_256hasher.finish();
                pwd.clear();
            }
        });

        if auth.pass_hash != u64::MAX {
            return auth;
        }

        root_ui().pop_skin();
        next_frame().await
    }
}