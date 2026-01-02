use anyhow::Result;
use std::fs;
use super::Config;

impl Config {
    pub fn parse() -> Result<Self> {
        let contents = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
