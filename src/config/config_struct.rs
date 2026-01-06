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
    pub battery: BatteryConfig,
    pub brightnessctl: BrightnessctlConfig,
    pub cputemp: CputempConfig,
    pub date: DateConfig,
    pub filesystem: FilesystemConfig,
    pub kernel: KernelConfig,
    pub loadavg: LoadavgConfig,
    pub memory: MemoryConfig,
    pub network: NetworkConfig,
    pub uptime: UptimeConfig,
    pub wpctl: WpctlConfig,
    pub xkeyboard: XkeyboardConfig,
    pub xwindow: XwindowConfig,
    pub xworkspaces: XworkspacesConfig,
}

#[derive(Deserialize)]
pub struct BatteryConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub name: String,
    pub format_charging: String,
    pub format_discharging: String,
    pub format_full: String,
}

#[derive(Deserialize)]
pub struct BrightnessctlConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub device_name: String,
    pub format: String,
}

#[derive(Deserialize)]
pub struct CputempConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format: String,
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
pub struct KernelConfig {
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

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub interface: String,
    pub format: String,
}

#[derive(Deserialize)]
pub struct UptimeConfig {
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

#[derive(Deserialize)]
pub struct XkeyboardConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
}

#[derive(Deserialize)]
pub struct XwindowConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub max_length: u32,
}

#[derive(Deserialize)]
pub struct XworkspacesConfig {
    pub interval: u32,
    pub icon: Option<String>,
    pub icon_color: Option<String>,
    pub format_active: String,
    pub format_empty: String,
    pub format_occupied: String,
    pub format_urgent: String,
    pub sepparator: String,
}
