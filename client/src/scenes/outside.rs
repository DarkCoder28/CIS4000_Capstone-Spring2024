use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};

pub async fn render_outside(theme: &Skin, asset_path: &str) -> String {
    let asset_path = asset_path.to_string();
    // Load Outside Map
    let mut map_path = asset_path.clone();
    map_path.push_str("pxArt.png");
    let map = load_texture(&map_path)
        .await
        .expect("Failed to load Outside Map");

    loop {
        root_ui().push_skin(&theme);
        //
        clear_background(GRAY);
        draw_texture_ex(
            &map,
            0.,
            0.,
            GRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        //
        root_ui().pop_skin();
        next_frame().await
    }

    String::new()
}
