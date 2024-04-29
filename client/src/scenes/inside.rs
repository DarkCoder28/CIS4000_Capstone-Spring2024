use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use ::glam::f32::vec2 as glam_vec2;
use common::{ClientState, UpdateEvent};
use git2::Object;
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use crate::{
    map_data::MapMeta,
    quest_data::{self, get_quest_data, GameData, Quest, Questline},
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

type UpdateQueue = Arc<Mutex<VecDeque<UpdateEvent>>>;
type SendQueue = Arc<Mutex<VecDeque<UpdateEvent>>>;

pub async fn render_inside(
    dialog_theme: &Skin,
    quests_theme: &Skin,
    asset_path: &str,
    map_data: &Vec<MapMeta>,
    game_data: &GameData,
    state: &mut ClientState,
    update_queue: UpdateQueue,
    send_queue: SendQueue,
) {
    let map_id = map_data
        .iter()
        .position(|v| &v.loc_id == &state.location)
        .unwrap();
    let map = map_data.get(map_id).unwrap();
    let asset_path = asset_path.to_string();
    let map_path = asset_path.clone();

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

    // let f = tiled_map.get_tile("1_collide", 5, 5);
    // match f {
    //     Some(x) => {
    //         info!("{}", x.tileset);
    //         info!("{}", x.id);
    //         info!("{}", x.attrs);
    //     }
    //     None => {}
    // }

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
    // let f: Vec<bool> = static_colliders.iter().map(|t| match t {Tile::Solid => true, _ => false}).collect();
    // info!("{:?}", f.as_slice());

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

    let mut relevant_objects = Vec::new();
    for obj in &game_data.object_locations {
        let mut relevant = false;
        if obj.loc_id == state.location {
            if obj.relevant_quest_ids.is_none() {
                relevant = true;
            } else {
                for quest in &state.current_quest_ids {
                    if obj.relevant_quest_ids.clone().unwrap().contains(quest) {
                        relevant = true;
                    }
                }
            }
        }
        if relevant {
            info!(
                "OBJECT (\n\tID: {}\n\tPOS: ({}, {})\n)",
                obj.object_id, obj.position.x, obj.position.y
            );
            relevant_objects.push(obj);
        }
    }

    let mut others: Vec<UpdateEvent> = Vec::new();

    loop {
        // Register ESC to leave building (this will change... esc will close the game and there will be a location to walk to to exit the building)
        if is_key_pressed(KeyCode::Escape) {
            state.location = "outside".to_string();
            let mut update = UpdateEvent::from_state_mut(state);
            update.pos = glam_vec2(0., 0.);
            update.speed = glam_vec2(0., 0.);
            send_queue.lock().unwrap().push_back(update);
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
            const PLAYER_SPRITE: u32 = 389;

            let pos = world.actor_pos(player.collider);
            tiled_map.spr(
                "interiors",
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
            let pos = world.actor_pos(player.collider);

            let player_speed_start = player.speed.clone();
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
            if player_speed_start != player.speed {
                // Tell server new player info
                state.pos = glam_vec2(pos.x, pos.y);
                state.speed = glam_vec2(player.speed.x, player.speed.y);
                let update = UpdateEvent::from_state_mut(state);
                send_queue.lock().unwrap().push_back(update);
            }

            world.move_h(player.collider, player.speed.x * 256. * get_frame_time());
            world.move_v(player.collider, player.speed.y * -256. * get_frame_time());
        }
        // Update Others Locations
        {
            let mut updates = update_queue.lock().unwrap();
            while let Some(update) = updates.pop_front() {
                // info!("{:#?}", &update);
                for i in 0..others.len() {
                    if others[i].username == update.username {
                        others.remove(i);
                    }
                }
                if !update.logout {
                    others.push(update);
                }
            }
        }
        // Quest Data
        let mut current_quests: Vec<Quest> = Vec::new();
        for quest_id in &state.current_quest_ids {
            current_quests.push(get_quest_data(*quest_id, &game_data.questlines).unwrap_or(
                Quest {
                    speaker: "".to_string(),
                    dialog: "ERROR FINDING QUEST".to_string(),
                    quest_id: Some(*quest_id),
                    quest_name: Some("Error finding quest".to_string()),
                    completion: None,
                },
            ));
        }
        // Render Quest Status Indicators
        {
            // info!("Current Quests: {:#?}", current_quests);
            root_ui().push_skin(&quests_theme);
            render_quest_status(&current_quests);
            root_ui().pop_skin();
        }
        // Render Dialog
        {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                for quest in current_quests {
                    let quest = quest.clone();
                    let completion_type = quest.completion.clone().unwrap().completion_type;
                    if completion_type.eq_ignore_ascii_case("interact") {
                        let object = match &quest.completion {
                            Some(x) => &x.interact_object_id,
                            None => &None,
                        };
                        if let Some(object) = object {
                            for obj in &game_data.object_locations {
                                if obj.object_id.eq_ignore_ascii_case(&object) {
                                    let obj_location = obj;
                                    let player_pos = world.actor_pos(player.collider) / 32.;
                                    if player_pos.abs_diff_eq(glam2mac(obj_location.position), 3.) {
                                        info!("Interacted with '{}'", object);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //
        root_ui().pop_skin();
        next_frame().await
    }
}

fn render_quest_status(quests: &Vec<Quest>) {
    if quests.len() <= 0 {
        return;
    }
    let mut names: Vec<String> = Vec::new();
    for quest in quests {
        if let Some(quest_name) = &quest.quest_name {
            names.push(quest_name.clone());
        }
    }
    let mut longest = &names[0];
    for name in &names {
        if name.len() > longest.len() {
            longest = name;
        }
    }
    let Vec2 {
        x: width,
        y: height,
    } = root_ui().calc_size(&longest);
    let margin = screen_width() * 0.025;
    let window_width = width + (margin * 2.);
    let window_height = (height * names.len() as f32) + (margin * 2.);

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
        let mut y_pos = margin / 2.;
        for name in names {
            ui.label(vec2(margin, y_pos), &name);
            y_pos += height;
        }
    });
}

fn render_quest_dialog(questline: Questline, quest_id: i16) {
    //
}

fn glam2mac(vec: ::glam::f32::Vec2) -> Vec2 {
    vec2(vec.x, vec.y)
}
