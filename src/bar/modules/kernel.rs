use async_trait::async_trait;
use std::fs;
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use crate::config::KernelConfig;
use crate::{Module, ModuleOutput};

/// Display date using a configured format
#[derive(Debug)]
pub struct KernelModule {
    tx: Sender<()>,
    interval: u64,
    kernel_info: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl KernelModule {
    pub fn new(config: &KernelConfig, tx: Sender<()>) -> Self {
        let format = config.format.clone();
        Self {
            tx,
            interval: config.interval,
            kernel_info: Mutex::new(kernel_info_from_string(&format)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

#[async_trait]
impl Module for KernelModule {
    async fn run(&self) {
        loop {
            *self.kernel_info.lock().await = kernel_info_from_string(&self.format);
            let _ = self.tx.send(());
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.kernel_info.lock().await.clone(),
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
