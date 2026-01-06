use crate::config::ModuleConfig;
use crate::Module;
use super::*;

pub fn build_modules(
    module_strings: &[String],
    config: &ModuleConfig,
) -> Vec<Box<dyn Module>> {
    module_strings
        .iter()
        .map(|s| {
            match s.as_str() {
                "battery" => Box::new(BatteryModule::new(&config.battery)) as Box<dyn Module>,
                "brightnessctl" => Box::new(BrightnessctlModule::new(&config.brightnessctl)) as Box<dyn Module>,
                "date" => Box::new(DateModule::new(&config.date)) as Box<dyn Module>,
                "filesystem" => Box::new(FilesystemModule::new(&config.filesystem)) as Box<dyn Module>,
                "loadavg" => Box::new(LoadavgModule::new(&config.loadavg)) as Box<dyn Module>,
                "memory" => Box::new(MemoryModule::new(&config.memory)) as Box<dyn Module>,
                "network" => Box::new(NetworkModule::new(&config.network)) as Box<dyn Module>,
                "uptime" => Box::new(UptimeModule::new(&config.uptime)) as Box<dyn Module>,
                "wpctl" => Box::new(WpctlModule::new(&config.wpctl)) as Box<dyn Module>,
                "xkeyboard" => Box::new(XkeyboardModule::new(&config.xkeyboard)) as Box<dyn Module>,
                "xwindow" => Box::new(XwindowModule::new(&config.xwindow)) as Box<dyn Module>,
                "xworkspaces" => Box::new(XworkspacesModule::new(&config.xworkspaces)) as Box<dyn Module>,
                _ => Box::new(InvalidModule::new(s)) as Box<dyn Module>,
            }
        })
        .collect()
}
