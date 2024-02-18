use std::net::TcpStream;

use common::ClientState;
use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};
use macroquad_tiled as tiled;
use macroquad_platformer::*;
use ::glam::f32::vec2 as glam_vec2;
use tungstenite::{stream::MaybeTlsStream, WebSocket, Message::Text};

use crate::map_data::MapMeta;

struct Player {
    collider: Actor,
    speed: Vec2,
}

#[allow(dead_code)]
struct Collision {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub async fn render_inside(theme: &Skin, asset_path: &str, map: &MapMeta, state: &mut ClientState, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let asset_path = asset_path.to_string();
    let mut map_path = asset_path.clone();
    map_path.push_str("maps/");

    let mut tilesets = Vec::new();
    for (name, path) in &map.tilemap_texture_mappings {
        let mut tileset_path = map_path.clone();
        tileset_path.push_str(path);
        let tileset = load_texture(&tileset_path).await.unwrap();
        tileset.set_filter(FilterMode::Nearest);
        tilesets.push((name.as_str(), tileset));
    }

    let mut tiled_map_path = map_path.clone();
    tiled_map_path.push_str(&map.tilemap_path);
    let tiled_map_json = load_string(&tiled_map_path).await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, tilesets.as_slice(), &[]).unwrap();

    let mut map_width = 0;
    let mut map_height = 0;
    let mut static_colliders = vec![];
    let mut first = true;
    let mut layer_order = vec![""; tiled_map.layers.len()];
    for (name, layer) in &tiled_map.layers {
        {
            let layer_id = name.split_once('_').unwrap().0;
            let layer_id: usize = layer_id.parse().unwrap();
            layer_order[layer_id] = name.as_str();
        }
        if name.to_ascii_lowercase().contains(&"collide".to_ascii_lowercase()) {
            if first {
                map_width = layer.width;
                map_height = layer.height;
            }
            for (x, y, tile) in tiled_map.tiles(name, None) {
                if first {
                    static_colliders.push(if tile.is_some() {
                        Tile::Solid
                    } else {
                        Tile::Empty
                    });
                } else {
                    if tile.is_some() {
                        static_colliders[((y*layer.width)+x) as usize] = Tile::Solid;
                    }
                }
            }
            first = false;
        }
    }
    // let f: Vec<bool> = static_colliders.iter().map(|t| match t {Tile::Solid => true, _ => false}).collect();
    // info!("{:?}", f.as_slice());

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 32., 32., map_width as usize, 1);

    let mut player = Player {
        collider: world.add_actor(vec2(5.*32., (map_height as f32-1.)*32.), 16, 16),
        speed: vec2(0.,0.),
    };

    loop {
        // Create Camera
        let camera = Camera2D::from_display_rect(Rect::new(0.0, screen_height(), screen_width(), screen_height()*-1.));
        set_camera(&camera);
        // Calculate Render Scale
        let map_size_x = map_width as f32 * 32.;
        let map_size_y = map_height as f32 * 32.;
        let scale_x = screen_width()/map_size_x;
        let scale_y = screen_height()/map_size_y;
        let scale = scale_x.min(scale_y);
        let scaled_map_size_x = map_size_x*scale;
        let scaled_map_size_y = map_size_y*scale;
        let map_offset_x = (screen_width()-scaled_map_size_x)/2.;
        let map_offset_y = (screen_height()-scaled_map_size_y)/2.;
        // Setup UI
        root_ui().push_skin(theme);
        clear_background(DARKGRAY);
        // Render Tiles
        for layer_name in &layer_order {
            tiled_map.draw_tiles(layer_name, Rect::new(map_offset_x, map_offset_y, scaled_map_size_x, scaled_map_size_y), None);
        }
        // Render Player
        {
            const PLAYER_SPRITE: u32 = 389;

            let pos = world.actor_pos(player.collider);
            tiled_map.spr("interiors", PLAYER_SPRITE, Rect::new(map_offset_x+(pos.x*scale), map_offset_y+(pos.y*scale), 32.*scale, 32.*scale));
        }
        // Calculate Player Movement
        {
            let pos = world.actor_pos(player.collider);
            // let collision = Collision {
            //     up: world.collide_check(player.collider, pos+vec2(0.,-1.)),
            //     down: world.collide_check(player.collider, pos+vec2(0.,1.)),
            //     left: world.collide_check(player.collider, pos+vec2(-1.,0.)),
            //     right: world.collide_check(player.collider, pos+vec2(1.,0.)),
            // };

            let player_speed_start = player.speed.clone();
            if is_key_down(KeyCode::Right) {
                player.speed.x = 1.;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -1.;
            } else {
                player.speed.x = 0.;
            }
            if is_key_down(KeyCode::Up) {
                player.speed.y = 1.;
            } else if is_key_down(KeyCode::Down) {
                player.speed.y = -1.;
            } else {
                player.speed.y = 0.;
            }
            if player_speed_start != player.speed {
                // Tell server new player info
                state.pos = glam_vec2(pos.x, pos.y);
                state.speed = glam_vec2(player.speed.x, player.speed.y);
                let state_ser = serde_json::to_string(state).expect("Failed to serialize state");
                let _ = socket.send(Text(state_ser));
                let _ = socket.flush();
            }

            world.move_h(player.collider, player.speed.x * 128. * get_frame_time());
            world.move_v(player.collider, player.speed.y * -128. * get_frame_time());
        }
        //
        root_ui().pop_skin();
        next_frame().await
    }
}