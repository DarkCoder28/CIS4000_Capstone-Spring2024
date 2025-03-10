use std::{net::TcpStream, sync::{Arc, Mutex}};

use common::{conn_lib::write_flush_client, ClientState};
use macroquad::{
    prelude::*,
    time,
    ui::{root_ui, Skin},
};
use openssl::ssl::SslStream;

use crate::{quest_data::GameData, ui::dialog::render_dialog};

pub async fn render_outside(
    theme: &Skin,
    asset_path: &str,
    outside_data: &Vec<crate::map_data::MapLocation>,
    game_data: &GameData,
    state: &mut ClientState,
    stream: Arc<Mutex<SslStream<TcpStream>>>
) -> String {
    let asset_path = asset_path.to_string();
    // Load Outside Map
    let mut map_path = asset_path.clone();
    map_path.push_str("pxArt.png");
    let map = load_texture(&map_path)
        .await
        .expect("Failed to load Outside Map");

    let esc_timeout = time::get_time();
    let mut open_time = get_time();
    let mut done_dialog = false;
    loop {
        // Register ESC to leave building
        if (time::get_time() - esc_timeout) > 0.25 && is_key_pressed(KeyCode::Escape) {
            state.location = "outside".to_string();
            let ser = serde_json::to_string(&state).unwrap();
            let _ = write_flush_client(stream.clone(), ser);
            return "exit".to_string();
        }
        //
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

        for location in outside_data {
            let location = draw_bounding_box(
                local_to_pixel(vec2(location.tl_corner.x, location.tl_corner.y)),
                local_to_pixel(vec2(location.br_corner.x, location.br_corner.y)),
                &location.label,
                mouse_pos.clone(),
                &location.loc_id,
            );
            exit = location;
            if exit.is_some() {
                state.location = exit.clone().unwrap();
                break;
            }
        }
        root_ui().pop_skin();
        root_ui().pop_skin();
        root_ui().pop_skin();
        if state.current_questline_id == 0 && state.current_quest_id == 0 && !done_dialog {
            let f = render_dialog(&game_data.questlines, open_time, state);
            open_time = f.1;
            done_dialog = f.0;
        }
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
    location: &str,
) -> Option<String> {
    let hover = mouse_pos.x > pos1.x
        && mouse_pos.x < pos2.x
        && mouse_pos.y > pos1.y
        && mouse_pos.y < pos2.y;
    if is_mouse_button_pressed(MouseButton::Left) && hover {
        return Some(location.to_string());
    }
    let rect_size = vec2(pos2.x - pos1.x, pos2.y - pos1.y);
    let rect_color = if !hover {
        Color::from_rgba(255, 0, 0, 64)
    } else {
        Color::from_rgba(255, 255, 0, 64)
    };
    draw_rectangle(pos1.x, pos1.y, rect_size.x, rect_size.y, rect_color);
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
