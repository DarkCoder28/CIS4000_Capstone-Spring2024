
use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};
use macroquad_tiled::load_map;

use crate::map_data::MapMeta;

pub async fn render_inside(theme: &Skin, asset_path: &str, map: &MapMeta, location: &str) {
    let asset_path = asset_path.to_string();

    let mut tilesets = Vec::new();
    for (name, path) in &map.tilemap_texture_mappings {
        let tileset = load_texture(path).await.unwrap();
        tileset.set_filter(FilterMode::Nearest);
        tilesets.push((name.as_str(), tileset));
    }

    let tiled_map_json = load_string(&map.tilemap_path).await.unwrap();
    let tiled_map = load_map(&tiled_map_json, tilesets.as_slice(), &[]).unwrap();

    loop {
        root_ui().push_skin(theme);
        //
        //
        root_ui().pop_skin();
        next_frame().await
    }
}