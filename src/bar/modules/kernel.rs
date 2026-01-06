use std::fs;
use crate::config::KernelConfig;
use crate::{Module, ModuleOutput};

/// Display date using a configured format
#[derive(Debug)]
pub struct KernelModule {
    kernel_info: String,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl KernelModule {
    pub fn new(config: &KernelConfig) -> Self {
        let format = config.format.clone();
        Self {
            kernel_info: kernel_info_from_string(&format),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

impl Module for KernelModule {
    fn update(&mut self) {
        self.kernel_info = kernel_info_from_string(&self.format);
    }

    fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.kernel_info.clone(),
        }
    }
}

fn read_trimmed(path: &str) -> String {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn kernel_info_from_string(format: &str) -> String {
    let ostype = read_trimmed("/proc/sys/kernel/ostype");
    let hostname = read_trimmed("/proc/sys/kernel/hostname");
    let osrelease = read_trimmed("/proc/sys/kernel/osrelease");
    let domainname = read_trimmed("/proc/sys/kernel/domainname");

    format
        .replace("{ostype}", &ostype)
        .replace("{hostname}", &hostname)
        .replace("{osrelease}", &osrelease)
        .replace("{domainname}", &domainname)
}
