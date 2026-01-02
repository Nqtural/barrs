use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub left: Vec<String>,
    pub center: Vec<String>,
    pub right: Vec<String>,
    pub sepparator: String,
    pub modules: ModuleConfig,
}

#[derive(Deserialize)]
pub struct ModuleConfig {
    pub date: DateConfig,
}

#[derive(Deserialize)]
pub struct DateConfig {
    pub interval: u32,
    pub format: String,
}

impl Config {
    pub fn parse() -> Result<Self> {
        let contents = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
