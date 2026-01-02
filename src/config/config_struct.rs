use serde::Deserialize;

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
