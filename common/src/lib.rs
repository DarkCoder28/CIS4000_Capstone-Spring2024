use glam::{f32::Vec2, vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientState {
    pub pos: Vec2,
    pub location: String,
    pub new_user: bool,
    pub authenticated: bool,
}

impl ClientState {
    pub fn new() -> ClientState {
        ClientState {
            pos: vec2(0., 0.),
            location: String::from("outside"),
            new_user: true,
            authenticated: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientAuth {
    pub username: String,
    pub pass_hash: u64,
}