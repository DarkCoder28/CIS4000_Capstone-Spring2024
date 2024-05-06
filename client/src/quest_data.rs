use common::ClientState;
use glam::f32::Vec2;
use macroquad::file::load_string;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Clone, Debug)]
pub struct GameData {
    pub questlines: Vec<Questline>,
    pub object_locations: Vec<ObjectLocation>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ObjectLocation {
    pub object_id: String,
    pub loc_id: String,
    pub sprite: TileId,
    pub position: Vec2,
    pub relevant_quest_ids: Option<Vec<u16>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileId {
    pub sprite_map: String,
    pub tile_id: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Questline {
    pub id: u16,
    pub quests: Vec<Quest>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Quest {
    pub speaker: String,
    pub dialog: String,
    pub quest_id: Option<u16>,
    pub quest_name: Option<String>,
    pub completion: Option<QuestCompletion>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuestCompletion {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub completion_type: String,
    #[serde(rename(serialize = "who", deserialize = "who"))]
    pub interact_object_id: Option<String>,
}

pub async fn import_quests(asset_path: &str) -> GameData {
    // Load Questlines
    let mut data_path = String::from(asset_path);
    data_path.push_str("questlines.json");
    let map_data = load_string(&data_path).await.unwrap();
    // Load Objects
    let mut data_path = String::from(asset_path);
    data_path.push_str("objects.json");
    let object_data = load_string(&data_path).await.unwrap();
    GameData {
        questlines: serde_json::from_str::<Vec<Questline>>(&map_data)
            .expect("Unable to deserialize map data"),
        object_locations: serde_json::from_str::<Vec<ObjectLocation>>(&object_data)
            .expect("Unable to deserialize object data"),
    }
}

pub fn get_quest_data(quest_id: u16, questlines: &Vec<Questline>) -> Option<Quest> {
    for questline in questlines {
        for quest in &questline.quests {
            if quest.quest_id.is_some_and(|id| id == quest_id) {
                return Some(quest.clone());
            }
        }
    }
    return None;
}

pub fn get_next_questline_id(questlines: &Vec<Questline>, current_questline:u16) -> u16 {
    let questline_ids: Vec<u16> = questlines.iter().map(|ql|ql.id).collect();
    let mut next_id = u16::MAX;
    for questline in questline_ids {
        if questline > current_questline && questline < next_id {
            next_id = questline;
        }
    }
    next_id
}