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
        let mut exit = String::new();
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
        // Draw St. Bernard's Bounding box
        let sb_rect_pos = local_to_pixel(Vec2::new(-0.45, -0.1));
        let sb_rect_pos2 = local_to_pixel(Vec2::new(-0.25, 0.1));
        let sb_rect_size = vec2(sb_rect_pos2.x-sb_rect_pos.x, sb_rect_pos2.y-sb_rect_pos.y);
        draw_rectangle(
            sb_rect_pos.x,
            sb_rect_pos.y,
            sb_rect_size.x,
            sb_rect_size.y,
            Color::from_rgba(255, 0, 0, 64),
        );
        let sb_text_pos = local_to_pixel(Vec2::new(-0.35, 0.));
        draw_text_ex("St. Bernard's", sb_text_pos.x, sb_text_pos.y, TextParams {
            color: WHITE,
            font_size: 32,
            rotation: -0.4363323,
            ..Default::default()
        });
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_pos = pixel_to_local(vec2(mouse_pos.0, mouse_pos.1));
            info!("Standard\t\t{}", mouse_pos);
            // Check St. Bernard's
            if mouse_pos.x > -0.45 && mouse_pos.x < -0.25 && mouse_pos.y > -0.1 && mouse_pos.y < 0.1 {
                info!("Navigating to St. B's");
                exit.push_str("sb");
            }
            // let sb_aligned_mouse = rotate_vec2(mouse_pos, 0.75);
            // info!("StB's\t\t{}", sb_aligned_mouse);
            // if sb_aligned_mouse.x > -0.65 && sb_aligned_mouse.x < -0.4 && sb_aligned_mouse.y > -0.7 && sb_aligned_mouse.y < -0.31 {
            //     info!("Activated");
            // }
        }
        //
        root_ui().pop_skin();
        if !exit.is_empty() {
            return exit;
        }
        next_frame().await
    }
}


#[allow(dead_code)]
fn rotate_vec2(x: Vec2, n: f32) -> Vec2 {
    let cos_n = n.cos();
    let sin_n = n.sin();
    let new_x = x.x * cos_n - x.y * sin_n;
    let new_y = x.x * sin_n + x.y * cos_n;
    Vec2::new(new_x, new_y)
}
#[allow(dead_code)]
fn local_to_pixel(local_coords: Vec2) -> Vec2 {
    let viewport_scale = vec2(screen_width(), screen_height());
    let viewport_position = vec2(screen_width() / 2.0, screen_height() / 2.0); // Assuming the viewport is centered

    (local_coords * viewport_scale) + viewport_position
}
#[allow(dead_code)]
fn pixel_to_local(pixel_coords: Vec2) -> Vec2 {
    let viewport_scale = vec2(screen_width(), screen_height());
    let viewport_position = vec2(screen_width() / 2.0, screen_height() / 2.0); // Assuming the viewport is centered

    (pixel_coords - viewport_position) / viewport_scale
}