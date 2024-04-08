pub mod conn_lib;

use glam::{f32::Vec2, vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientState {
    pub username: String,
    pub new_user: bool,
    pub authenticated: bool,
    pub pos: Vec2,
    pub speed: Vec2,
    pub location: String,
    pub current_quests: Vec<String>,
    pub complete_quests: Vec<String>,
}

impl ClientState {
    pub fn new(uname: &str) -> ClientState {
        ClientState {
            username: String::from(uname),
            pos: vec2(0., 0.),
            speed: vec2(0. ,0.),
            location: String::from("outside"),
            new_user: true,
            authenticated: true,
            current_quests: Vec::from(["GettingStarted".to_string()]),
            complete_quests: Vec::new(),
        }
    }
    pub fn apply_update(&mut self, update: &UpdateEvent) {
        self.pos = update.pos;
        self.speed = update.speed;
        self.location = update.location.clone();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateEvent {
    pub username: String,
    pub pos: Vec2,
    pub speed: Vec2,
    pub location: String,
    pub logout: bool,
}
impl UpdateEvent {
    pub fn from_state(state: &ClientState) -> UpdateEvent {
        UpdateEvent {
            username: state.username.clone(),
            pos: state.pos,
            speed: state.speed,
            location: state.location.clone(),
            logout: false,
        }
    }
    pub fn from_state_mut(state: &mut ClientState) -> UpdateEvent {
        UpdateEvent {
            username: state.username.clone(),
            pos: state.pos.clone(),
            speed: state.speed.clone(),
            location: state.location.clone(),
            logout: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientAuth {
    pub username: String,
    pub pass_hash: u64,
}