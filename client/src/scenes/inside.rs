use ::glam::f32::vec2 as glam_vec2;
use common::ClientState;
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Skin},
};
use macroquad_platformer::*;
use macroquad_tiled as tiled;
// use openssl::ssl::SslStream;

use crate::{
    map_data::MapMeta,
    quest_data::{get_quest_data, GameData, Quest, Questline},
    ui::dialog::render_dialog,
};

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

const DEBUG: bool = true;

pub async fn render_inside(
    _dialog_theme: &Skin,
    quests_theme: &Skin,
    asset_path: &str,
    map_data: &Vec<MapMeta>,
    game_data: &GameData,
    state: &mut ClientState,
) {
    let map_id = map_data
        .iter()
        .position(|v| &v.loc_id == &state.location)
        .unwrap();
    let map = map_data.get(map_id).unwrap();
    let asset_path = asset_path.to_string();
    let mut map_path = asset_path.clone();
    map_path.push_str("maps/");

    info!("Load Tilesets");
    let mut tilesets = Vec::new();
    for (name, path) in &map.tilemap_texture_mappings {
        let mut tileset_path = map_path.clone();
        tileset_path.push_str(path);
        let tileset = load_texture(&tileset_path).await.unwrap();
        tileset.set_filter(FilterMode::Nearest);
        tilesets.push((name.as_str(), tileset));
    }

    info!("Load Tilemap");
    let mut tiled_map_path = map_path.clone();
    tiled_map_path.push_str(&map.tilemap_path);
    let tiled_map_json = load_string(&tiled_map_path).await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, tilesets.as_slice(), &[]).unwrap();

    info!("Calculate Collisions & Layer Order");
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
        if name
            .to_ascii_lowercase()
            .contains(&"collide".to_ascii_lowercase())
        {
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
                        static_colliders[((y * layer.width) + x) as usize] = Tile::Solid;
                    }
                }
            }
            first = false;
        }
    }

    info!("Add collision to world");
    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 32., 32., map_width as usize, 1);

    // Setup Player
    let player_pos = if state.pos.eq(&glam_vec2(0., 0.)) {
        glam2mac(map.spawn_location) * vec2(32., 32.)
    } else {
        glam2mac(state.pos)
    };
    let mut player = Player {
        collider: world.add_actor(player_pos, 28, 28),
        speed: vec2(0., 0.),
    };

    let mut open_time = get_time();
    let mut done_dialog = true;

    loop {
        // Relevant Objects
        let mut relevant_objects = Vec::new();
        for obj in &game_data.object_locations {
            let mut relevant = false;
            if obj.loc_id == state.location {
                if obj.relevant_quest_ids.is_none() {
                    relevant = true;
                } else {
                    let relevant_quest_ids = obj.relevant_quest_ids.clone().unwrap();
                    if relevant_quest_ids.contains(&(state.current_questline_id+state.current_quest_id)) {
                        relevant = true;
                    }
                }
            }
            if relevant {
                relevant_objects.push(obj);
            }
        }
        // Register ESC to leave building (this will change... esc will close the game and there will be a location to walk to to exit the building)
        if is_key_pressed(KeyCode::Escape) {
            state.location = String::from("outside");
            break;
        }
        // Create Camera
        let camera = Camera2D::from_display_rect(Rect::new(
            0.0,
            screen_height(),
            screen_width(),
            screen_height() * -1.,
        ));
        set_camera(&camera);
        // Calculate Render Scale
        let map_size_x = map_width as f32 * 32.;
        let map_size_y = map_height as f32 * 32.;
        let scale_x = screen_width() / map_size_x;
        let scale_y = screen_height() / map_size_y;
        let scale = scale_x.min(scale_y);
        let scaled_map_size_x = map_size_x * scale;
        let scaled_map_size_y = map_size_y * scale;
        let map_offset_x = (screen_width() - scaled_map_size_x) / 2.;
        let map_offset_y = (screen_height() - scaled_map_size_y) / 2.;
        // Setup UI
        clear_background(BLACK);
        // Render Tiles
        for layer_name in &layer_order {
            tiled_map.draw_tiles(
                layer_name,
                Rect::new(
                    map_offset_x,
                    map_offset_y,
                    scaled_map_size_x,
                    scaled_map_size_y,
                ),
                None,
            );
        }
        // Render Objects
        {
            for obj in &relevant_objects {
                tiled_map.spr(
                    &obj.sprite.sprite_map,
                    obj.sprite.tile_id,
                    Rect::new(
                        map_offset_x + ((obj.position.x * 32.) * scale),
                        map_offset_y + ((obj.position.y * 32.) * scale),
                        32. * scale,
                        32. * scale,
                    ),
                );
            }
        }
        // Render Player
        {
            const PLAYER_SPRITE: u32 = 0;

            let pos = world.actor_pos(player.collider);
            tiled_map.spr(
                "objects",
                PLAYER_SPRITE,
                Rect::new(
                    map_offset_x + ((pos.x - 2.) * scale),
                    map_offset_y + ((pos.y - 2.) * scale),
                    32. * scale,
                    32. * scale,
                ),
            );
        }
        // Debug Locator
        if DEBUG {
            if is_key_pressed(KeyCode::P) {
                let pos = world.actor_pos(player.collider);
                let pos_x = (pos.x / scale / 32.) as i32;
                let pos_y = (pos.y / scale / 32.) as i32;
                info!("Current Block Location: ({}, {})", pos_x, pos_y);
            }
        }
        // Calculate Player Movement
        {
            // let pos = world.actor_pos(player.collider);
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                player.speed.x = 1.;
            } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                player.speed.x = -1.;
            } else {
                player.speed.x = 0.;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                player.speed.y = 1.;
            } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                player.speed.y = -1.;
            } else {
                player.speed.y = 0.;
            }

            world.move_h(player.collider, player.speed.x * 256. * get_frame_time());
            world.move_v(player.collider, player.speed.y * -256. * get_frame_time());
        }
        // Quest Data
        let current_quest = get_quest_data(&game_data.questlines, &state)
            .unwrap_or(Quest {
                speaker: "".to_string(),
                dialog: "ERROR FINDING QUEST".to_string(),
                quest_id: Some(state.current_quest_id),
                quest_name: Some("Error finding quest".to_string()),
                completion: None,
            });
        // Render Quest Status Indicators
        {
            root_ui().push_skin(&quests_theme);
            render_quest_status(&game_data.questlines, &state);
            root_ui().pop_skin();
        }
        // Render Dialog
        {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                let quest = current_quest.clone();
                let completion_type = quest.completion.clone().unwrap().completion_type;
                if completion_type.eq_ignore_ascii_case("interact") {
                    let object = quest.completion.unwrap().interact_object_id;
                    if object.is_some() {
                        let object_id = object.unwrap();
                        for obj in &relevant_objects {
                            if obj.object_id.eq_ignore_ascii_case(&object_id) {
                                // let obj_location = obj;
                                let player_pos = world.actor_pos(player.collider) / 32.;
                                if player_pos.distance(glam2mac(obj.position))< 3. {
                                    info!("Interacted with '{}'", object_id);
                                    done_dialog = false;
                                    state.complete_quest_ids.push(state.current_questline_id+state.current_quest_id);
                                    state.dialog_offset += 1;
                                }
                            }
                        }
                    }
                }
            }
            if !done_dialog {
                let f = render_dialog(&game_data.questlines, open_time, state);
                open_time = f.1;
                done_dialog = f.0;
            }
        }
        //
        root_ui().pop_skin();
        next_frame().await
    }
}

fn render_quest_status(questlines: &Vec<Questline>, state: &ClientState) {
    let quest = questlines
        .iter()
        .find(|ql| ql.id == state.current_questline_id)
        .unwrap()
        .quests
        .iter()
        .find(|q| q.quest_id.is_some() && q.quest_id.unwrap() == state.current_quest_id)
        .unwrap();
    let name = quest.quest_name.clone().unwrap();

    let Vec2 {
        x: width,
        y: height,
    } = root_ui().calc_size(&name);
    let margin = screen_width() * 0.025;
    let window_width = width + (margin * 2.);
    let window_height = height + (margin * 2.);

    root_ui().move_window(
        1010,
        Vec2::new(screen_width() - window_width - margin, margin),
    );
    widgets::Window::new(
        1010,
        Vec2::new(screen_width() - window_width - margin, margin),
        Vec2::new(window_width, window_height),
    )
    .label("Quests")
    .titlebar(true)
    .close_button(false)
    .ui(&mut root_ui(), |ui| {
        ui.label(None, &name);
    });
}

fn glam2mac(vec: ::glam::f32::Vec2) -> Vec2 {
    vec2(vec.x, vec.y)
}
