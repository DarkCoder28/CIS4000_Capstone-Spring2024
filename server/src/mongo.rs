use std::env;

use common::{ClientAuth, ClientState};
use mongodb::{error::Error, options::{ClientOptions, ResolverConfig}, Client};



pub async fn connect() -> Result<Client, Error> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    Client::with_options(options)
}

pub async fn auth_and_get_client(auth: &ClientAuth) -> ClientState {
    ClientState::new(&auth.username)
}