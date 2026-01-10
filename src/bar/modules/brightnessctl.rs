use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::process::Command;
use crate::config::BrightnessctlConfig;
use crate::{Module, ModuleOutput};

/// Display brightness info about a given device using a configured format
#[derive(Debug)]
pub struct BrightnessctlModule {
    interval: u64,
    current_brightness: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    device_name: String,
    format: String,
}

impl BrightnessctlModule {
    pub fn new(config: &BrightnessctlConfig) -> Self {
        let device_name = config.device_name.clone();
        let format = config.format.clone();
        Self {
            interval: config.interval,
            current_brightness: Mutex::new(brightness_from_string(
                &device_name,
                &format,
            )),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            device_name,
            format,
        }
    }
}

#[async_trait]
impl Module for BrightnessctlModule {
    async fn run(&self) {
        loop {
            {
                *self.current_brightness.lock().await = brightness_from_string(
                    &self.device_name,
                    &self.format,
                );
            }
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_brightness.lock().await.clone(),
        }
    }
}

fn brightness_from_string(device_name: &str, format: &str) -> String {
    let output = Command::new("brightnessctl")
        .args(["i", "-d", device_name])
        .output();

    let info = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).into_owned(),
        Err(e) => {
            return format!("error: failed to execute brightnessctl: {}", e);
        }
    };

    let mut current_percent: Option<&str> = None;

    for line in info.lines() {
        if line.contains("Current brightness") {
            let start = line.find('(');
            let end = line.find(')');

            if let (Some(start), Some(end)) = (start, end)
            && start + 1 < end {
                    current_percent = Some(&line[start + 1..end]);
            }

            break;
        }
    }

    match current_percent {
        Some(value) => format.replace("{%}", value),
        None => "error: could not parse current brightness".to_string(),
    }
}
