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
    pub filesystem: FilesystemConfig,
    pub loadavg: LoadavgConfig,
    pub memory: MemoryConfig,
    pub wpctl: WpctlConfig,
}

#[derive(Deserialize)]
pub struct DateConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
}
#[derive(Deserialize)]
pub struct FilesystemConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
    pub mountpoint: String,
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

#[derive(Deserialize)]
pub struct WpctlConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
    pub format_muted: String,
}
