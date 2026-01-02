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
                "date" => Box::new(DateModule::new(&config.date)) as Box<dyn Module>,
                "loadavg" => Box::new(LoadavgModule::new(&config.loadavg)) as Box<dyn Module>,
                "memory" => Box::new(MemoryModule::new(&config.memory)) as Box<dyn Module>,
                "wpctl" => Box::new(WpctlModule::new(&config.wpctl)) as Box<dyn Module>,
                _ => Box::new(InvalidModule::new(s)) as Box<dyn Module>,
            }
        })
        .collect()
}
