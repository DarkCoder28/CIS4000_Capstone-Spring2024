use glam::f32::Vec2;
use macroquad::file::load_string;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Maps {
    pub outside: Vec<MapLocation>,
    pub insides: Vec<MapMeta>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MapLocation {
    pub loc_id: String,
    pub tl_corner: Vec2,
    pub br_corner: Vec2,
    pub label: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MapMeta {
    pub loc_id: String,
    pub tilemap_path: String,
    pub tilemap_texture_mappings: Vec<(String, String)>,
    pub spawn_location: Vec2
}

pub async fn import_data(asset_path: &str) -> Maps {
    let mut data_path = String::from(asset_path);
    data_path.push_str("maps/map_data.json");
    let map_data = load_string(&data_path).await.unwrap();
    serde_json::from_str::<Maps>(&map_data).expect("Unable to deserialize map data")
}