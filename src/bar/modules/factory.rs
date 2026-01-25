use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::config::ModuleConfig;
use crate::Module;
use super::*;

pub fn build_modules(
    module_strings: &[String],
    config: &ModuleConfig,
    tx: &Sender<()>,
) -> Vec<Arc<dyn Module + Send + Sync>> {
    module_strings
        .iter()
        .map(|s| {
            match s.as_str() {
                "battery" =>       Arc::new(BatteryModule::new(      &config.battery      , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "brightnessctl" => Arc::new(BrightnessctlModule::new(&config.brightnessctl, tx.clone())) as Arc<dyn Module + Send + Sync>,
                "cpu" =>           Arc::new(CpuModule::new(          &config.cpu          , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "cputemp" =>       Arc::new(CputempModule::new(      &config.cputemp      , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "date" =>          Arc::new(DateModule::new(         &config.date         , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "filesystem" =>    Arc::new(FilesystemModule::new(   &config.filesystem   , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "kernel" =>        Arc::new(KernelModule::new(       &config.kernel       , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "loadavg" =>       Arc::new(LoadavgModule::new(      &config.loadavg      , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "memory" =>        Arc::new(MemoryModule::new(       &config.memory       , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "network" =>       Arc::new(NetworkModule::new(      &config.network      , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "uptime" =>        Arc::new(UptimeModule::new(       &config.uptime       , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "wpctl" =>         Arc::new(WpctlModule::new(        &config.wpctl        , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "xkeyboard" =>     Arc::new(XkeyboardModule::new(    &config.xkeyboard    , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "xwindow" =>       Arc::new(XwindowModule::new(      &config.xwindow      , tx.clone())) as Arc<dyn Module + Send + Sync>,
                "xworkspaces" =>   Arc::new(XworkspacesModule::new(  &config.xworkspaces  , tx.clone())) as Arc<dyn Module + Send + Sync>,
                _ =>               Arc::new(InvalidModule::new(      s                    , tx.clone())) as Arc<dyn Module + Send + Sync>,
            }
        })
        .collect()
}
