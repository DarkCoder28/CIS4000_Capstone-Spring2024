use std::fs;

use anyhow::Result;

pub fn load_servers(config_dir:&str) -> Result<Vec<String>> {
    let mut config_path = String::from(config_dir);
    config_path.push_str("servers.json");
    
    let contents = fs::read_to_string(&config_path);
    let contents = match contents {
        Ok(string) => string,
        Err(_error) => {
            let servers = Vec::from([String::from("home.thesheerans.com:3333")]);
            save_servers(&servers, &config_dir)?;
            return Ok(servers);
        }
    };
    let servers: Vec<String> = serde_json::from_str(&contents)?;
    Ok(servers)
}

pub fn save_servers(servers: &Vec<String>, config_dir:&str) -> Result<()> {
    let mut config_path = String::from(config_dir);
    config_path.push_str("servers.json");

    let serialized = serde_json::to_string_pretty(servers)?;
    let dir_creation = fs::create_dir_all(config_dir);
    if matches!(dir_creation, Err(_)) {
        panic!("Error creating config directory!");
    }
    let file_creation = fs::write(&config_path, serialized);
    if matches!(file_creation, Err(_)) {
        panic!("Error writing config file!");
    }
    Ok(())
}