use crate::enums::NavLocations;
use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};

pub async fn render_outside(theme: &Skin, asset_path: &str) -> NavLocations {
    let asset_path = asset_path.to_string();
    // Load Outside Map
    let mut map_path = asset_path.clone();
    map_path.push_str("pxArt.png");
    let map = load_texture(&map_path)
        .await
        .expect("Failed to load Outside Map");

    loop {
        let mut exit = None;
        root_ui().push_skin(&theme);
        let mouse_pos = mouse_position();
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);
        //
        if is_mouse_button_pressed(MouseButton::Left) {
            info!("{}", pixel_to_local(mouse_pos));
        }
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

        for (loc, pos1, pos2, label) in [
            (
                NavLocations::SaintBernards,
                local_to_pixel(vec2(-0.45, -0.1)),
                local_to_pixel(vec2(-0.25, 0.1)),
                "St. Bernard's"
            ),
            (
                NavLocations::Library,
                local_to_pixel(vec2(-0.094033616, -0.43883497)),
                local_to_pixel(vec2(0.14495799, -0.16796117)),
                "Library"
            ),
            (
                NavLocations::UniversityHall,
                local_to_pixel(vec2(0.13596639, -0.11941747)),
                local_to_pixel(vec2(0.34817647, 0.36407766)),
                "University Hall"
            )
        ] {
            let location = draw_bounding_box(pos1, pos2, label, mouse_pos.clone(), loc);
            exit = location;
            if exit.is_some() {
                break;
            }
        }

        //
        root_ui().pop_skin();
        if exit.is_some() {
            return exit.unwrap();
        }
        next_frame().await
    }
}

fn draw_bounding_box(
    pos1: Vec2,
    pos2: Vec2,
    label: &str,
    mouse_pos: Vec2,
    location: NavLocations,
) -> Option<NavLocations> {
    let hover = 
            mouse_pos.x > pos1.x
        &&  mouse_pos.x < pos2.x
        &&  mouse_pos.y > pos1.y
        &&  mouse_pos.y < pos2.y;
    if is_mouse_button_pressed(MouseButton::Left) && hover {
        return Some(location);
    }
    let rect_size = vec2(pos2.x - pos1.x, pos2.y - pos1.y);
    let rect_color = if !hover {
        Color::from_rgba(255, 0, 0, 64)
    } else {
        Color::from_rgba(255, 255, 0, 64)
    };
    draw_rectangle(
        pos1.x,
        pos1.y,
        rect_size.x,
        rect_size.y,
        rect_color,
    );
    let text_pos = Vec2::new(pos1.x + (rect_size.x / 2.), pos1.y + (rect_size.y / 2.));
    draw_text_ex(
        label,
        text_pos.x,
        text_pos.y,
        TextParams {
            color: WHITE,
            font_size: 32,
            rotation: -0.4363323,
            ..Default::default()
        },
    );
    None
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
