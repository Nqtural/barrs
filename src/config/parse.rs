use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use super::Config;

impl Config {
    pub fn parse() -> Result<Self> {
        let mut paths = Vec::new();

        paths.push(PathBuf::from("config.toml"));

        if let Some(xdg_config_home) = dirs::config_dir() {
            paths.push(xdg_config_home.join("barrs").join("config.toml"));
        }

        if let Some(home_dir) = dirs::home_dir() {
            paths.push(home_dir.join(".config").join("barrs").join("config.toml"));
        }

        for path in paths {
            if path.exists() {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("failed to read config file at {:?}", path))?;
                let config: Config = toml::from_str(&contents)
                    .map_err(|e| anyhow::anyhow!(
                        "failed to parse config file at {:?}:\n{}",
                        path,
                        e,
                    ))?;
                return Ok(config);
            }
        }

        anyhow::bail!("no config file found in any of the expected locations");
    }
}
