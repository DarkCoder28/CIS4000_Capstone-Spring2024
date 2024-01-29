use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;

pub async fn connect_mongo() -> Result<Client, Error> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    Client::with_options(options)
}
