pub mod conn_lib;

use glam::{f32::Vec2, vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientState {
    pub username: String,
    pub authenticated: bool,
    pub pos: Vec2,
    pub speed: Vec2,
    pub location: String,
    pub current_quest_ids: Vec<u16>,
    pub complete_quest_ids: Vec<u16>,
}

impl ClientState {
    pub fn new(uname: &str) -> ClientState {
        ClientState {
            username: String::from(uname),
            pos: vec2(0., 0.),
            speed: vec2(0. ,0.),
            location: String::from("outside"),
            authenticated: true,
            current_quest_ids: Vec::from([1]),
            complete_quest_ids: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserStore {
    pub username: String,
    pub pass_hash: u64,
    pub state: ClientState,
}

impl UserStore {
    pub fn new(username: &str, pass_hash: u64) -> UserStore {
        UserStore { username: String::from(username), pass_hash: pass_hash, state: ClientState::new(username) }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientAuth {
    pub username: String,
    pub pass_hash: u64,
}