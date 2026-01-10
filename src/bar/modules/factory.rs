use std::sync::Arc;
use crate::config::ModuleConfig;
use crate::Module;
use super::*;

pub fn build_modules(
    module_strings: &[String],
    config: &ModuleConfig,
) -> Vec<Arc<dyn Module + Send + Sync>> {
    module_strings
        .iter()
        .map(|s| {
            match s.as_str() {
                "battery" => Arc::new(BatteryModule::new(&config.battery)) as Arc<dyn Module + Send + Sync>,
                "brightnessctl" => Arc::new(BrightnessctlModule::new(&config.brightnessctl)) as Arc<dyn Module + Send + Sync>,
                "cpu" => Arc::new(CpuModule::new(&config.cpu)) as Arc<dyn Module + Send + Sync>,
                "cputemp" => Arc::new(CputempModule::new(&config.cputemp)) as Arc<dyn Module + Send + Sync>,
                "date" => Arc::new(DateModule::new(&config.date)) as Arc<dyn Module + Send + Sync>,
                "filesystem" => Arc::new(FilesystemModule::new(&config.filesystem)) as Arc<dyn Module + Send + Sync>,
                "kernel" => Arc::new(KernelModule::new(&config.kernel)) as Arc<dyn Module + Send + Sync>,
                "loadavg" => Arc::new(LoadavgModule::new(&config.loadavg)) as Arc<dyn Module + Send + Sync>,
                "memory" => Arc::new(MemoryModule::new(&config.memory)) as Arc<dyn Module + Send + Sync>,
                "network" => Arc::new(NetworkModule::new(&config.network)) as Arc<dyn Module + Send + Sync>,
                "uptime" => Arc::new(UptimeModule::new(&config.uptime)) as Arc<dyn Module + Send + Sync>,
                "wpctl" => Arc::new(WpctlModule::new(&config.wpctl)) as Arc<dyn Module + Send + Sync>,
                "xkeyboard" => Arc::new(XkeyboardModule::new(&config.xkeyboard)) as Arc<dyn Module + Send + Sync>,
                "xwindow" => Arc::new(XwindowModule::new(&config.xwindow)) as Arc<dyn Module + Send + Sync>,
                "xworkspaces" => Arc::new(XworkspacesModule::new(&config.xworkspaces)) as Arc<dyn Module + Send + Sync>,
                _ => Arc::new(InvalidModule::new(s)) as Arc<dyn Module + Send + Sync>,
            }
        })
        .collect()
}
