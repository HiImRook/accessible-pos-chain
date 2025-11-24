use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub listen_addr: String,
    pub rpc_addr: String,
    pub bootstrap_nodes: Vec<String>,
    pub genesis: HashMap<String, u64>,
    pub validators: HashMap<String, u64>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
