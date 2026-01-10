use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::fs;
use crate::config::LoadavgConfig;
use crate::{Module, ModuleOutput};

/// Display average CPU load using a configured format
#[derive(Debug)]
pub struct LoadavgModule {
    interval: u64,
    current_loadavg: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl LoadavgModule {
    pub fn new(config: &LoadavgConfig) -> Self {
        let format = config.format.clone();
        Self {
            interval: config.interval,
            current_loadavg: Mutex::new(loadavg_from_string(&format)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

#[async_trait]
impl Module for LoadavgModule {
    async fn run(&self) {
        loop {
            *self.current_loadavg.lock().await = loadavg_from_string(&self.format);
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_loadavg.lock().await.clone(),
        }
    }
}

fn loadavg_from_string(format: &str) -> String {
    let contents = match fs::read_to_string("/proc/loadavg") {
        Ok(s) => s,
        Err(_) => return "load: n/a".into(),
    };

    let parts: Vec<&str> = contents.split_whitespace().collect();
    if parts.len() < 4 {
        return "load: n/a".into();
    }

    let one = parts[0].parse::<f64>().unwrap_or(0.0);
    let five = parts[1].parse::<f64>().unwrap_or(0.0);
    let fifteen = parts[2].parse::<f64>().unwrap_or(0.0);

    let total_processes = parts[3]
        .split('/')
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    let mut output = format.to_string();
    output = output.replace("{1m}", &format!("{:.2}", one));
    output = output.replace("{5m}", &format!("{:.2}", five));
    output = output.replace("{15m}", &format!("{:.2}", fifteen));
    output = output.replace("{total}", &format!("{}", total_processes));

    output
}
