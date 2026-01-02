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
    pub loadavg: LoadavgConfig,
    pub memory: MemoryConfig,
}

#[derive(Deserialize)]
pub struct DateConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
}

#[derive(Deserialize)]
pub struct LoadavgConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
}
#[derive(Deserialize)]
pub struct MemoryConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
}
