use glam::{f32::Vec2, vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientState {
    pub username: String,
    pub new_user: bool,
    pub authenticated: bool,
    pub pos: Vec2,
    pub location: String,
    pub current_quests: Vec<String>,
    pub complete_quests: Vec<String>,
}

impl ClientState {
    pub fn new(uname: &str) -> ClientState {
        ClientState {
            username: String::from(uname),
            pos: vec2(0., 0.),
            location: String::from("outside"),
            new_user: true,
            authenticated: true,
            current_quests: Vec::from(["GettingStarted".to_string()]),
            complete_quests: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientAuth {
    pub username: String,
    pub pass_hash: u64,
}