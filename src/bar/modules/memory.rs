use std::fs;
use crate::config::MemoryConfig;
use crate::{Module, ModuleOutput};

/// Display sytem memory usage using a configured format
#[derive(Debug)]
pub struct MemoryModule {
    current_usage: String,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

/// Display system memory usage using a configured format
impl MemoryModule {
    pub fn new(config: &MemoryConfig) -> Self {
        let format = config.format.clone();
        Self {
            current_usage: usage_from_string(&format),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

impl Module for MemoryModule {
    fn update(&mut self) {
        self.current_usage = usage_from_string(&self.format);
    }

    fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_usage.clone(),
        }
    }
}

fn usage_from_string(format: &str) -> String {
    let content = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return "mem: n/a".into(),
    };

    let mut total_kb = 0u64;
    let mut available_kb = 0u64;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        }
        if line.starts_with("MemAvailable:") {
            available_kb = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        }
    }

    let used_kb = total_kb.saturating_sub(available_kb);

    let percent = if total_kb > 0 {
        used_kb as f64 / total_kb as f64 * 100.0
    } else {
        0.0
    };
    let mb = used_kb as f64 / 1000.0;
    let mib = used_kb as f64 / 1024.0;
    let gb = used_kb as f64 / 1_000_000.0;
    let gib = used_kb as f64 / 1_048_576.0;

    format
        .replace("{%}", &format!("{:.0}", percent))
        .replace("{mb}", &format!("{:.0}", mb))
        .replace("{mib}", &format!("{:.0}", mib))
        .replace("{gb}", &format!("{:.2}", gb))
        .replace("{gib}", &format!("{:.2}", gib))
}

