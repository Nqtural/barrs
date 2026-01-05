use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub left: Vec<String>,
    pub center: Vec<String>,
    pub right: Vec<String>,
    pub separator: String,
    pub frontend: String,
    pub modules: ModuleConfig,
}

#[derive(Deserialize)]
pub struct ModuleConfig {
    pub date: DateConfig,
}

#[derive(Deserialize)]
pub struct DateConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
}
