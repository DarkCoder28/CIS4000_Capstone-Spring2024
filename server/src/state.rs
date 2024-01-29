use std::sync::Arc;

use mongodb::Client;
use webauthn_rs::Webauthn;

#[derive(Clone)]
pub struct AppState {
    pub mongo: Client,
    pub webauthn: Arc<Webauthn>,
}