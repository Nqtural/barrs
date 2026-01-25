use async_trait::async_trait;
use std::fs;
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use crate::config::UptimeConfig;
use crate::{Module, ModuleOutput};

/// Display uptime using a configured format
#[derive(Debug)]
pub struct UptimeModule {
    tx: Sender<()>,
    interval: u64,
    current_uptime: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl UptimeModule {
    pub fn new(config: &UptimeConfig, tx: Sender<()>) -> Self {
        let format = config.format.clone();
        Self {
            tx,
            interval: config.interval,
            current_uptime: Mutex::new(uptime_from_string(&format)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

#[async_trait]
impl Module for UptimeModule {
    async fn run(&self) {
        loop {
            *self.current_uptime.lock().await = uptime_from_string(&self.format);
            let _ = self.tx.send(());
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_uptime.lock().await.clone(),
        }
    }
}

fn uptime_from_string(format: &str) -> String {
    let uptime_contents = fs::read_to_string("/proc/uptime")
        .unwrap_or_else(|_| "0.0 0.0".to_string());

    let uptime_seconds: u64 = uptime_contents
        .split_whitespace()
        .next()
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let total_seconds = uptime_seconds;
    let total_minutes = total_seconds / 60;
    let total_hours = total_seconds / 3600;
    let total_days = total_seconds / 86400;

    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = (total_seconds / 3600) % 24;
    let days = total_days;

    let total_seconds = total_seconds.to_string();
    let total_minutes = total_minutes.to_string();
    let total_hours = total_hours.to_string();
    let total_days = total_days.to_string();

    let seconds = seconds.to_string();
    let minutes = minutes.to_string();
    let hours = hours.to_string();
    let days = days.to_string();

    format
        .replace("{total_days}", &total_days)
        .replace("{total_hours}", &total_hours)
        .replace("{total_minutes}", &total_minutes)
        .replace("{total_seconds}", &total_seconds)
        .replace("{days}", &days)
        .replace("{hours}", &hours)
        .replace("{minutes}", &minutes)
        .replace("{seconds}", &seconds)
}
