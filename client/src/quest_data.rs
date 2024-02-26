use macroquad::file::load_string;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Questline {
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
    pub completion_type: String
}


pub async fn import_quests(asset_path: &str) -> Vec<Questline> {
    let mut data_path = String::from(asset_path);
    data_path.push_str("questlines.json");
    let map_data = load_string(&data_path).await.unwrap();
    serde_json::from_str::<Vec<Questline>>(&map_data).expect("Unable to deserialize map data")
}