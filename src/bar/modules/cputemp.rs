use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::fs;
use std::path::Path;
use crate::config::CputempConfig;
use crate::{Module, ModuleOutput};

/// Display temperature of CPU using a configured format
#[derive(Debug)]
pub struct CputempModule {
    interval: u64,
    current_temp: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl CputempModule {
    pub fn new(config: &CputempConfig) -> Self {
        let format = config.format.clone();
        Self {
            interval: config.interval,
            current_temp: Mutex::new(cputemp_from_string(&format)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

#[async_trait]
impl Module for CputempModule {
    async fn run(&self) {
        loop {
            *self.current_temp.lock().await = cputemp_from_string(&self.format);
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_temp.lock().await.clone(),
        }
    }
}

fn cputemp_from_string(format: &str) -> String {
    fn read_cpu_temp_celsius() -> Option<f64> {
        let thermal_path = Path::new("/sys/class/thermal");

        let entries = fs::read_dir(thermal_path).ok()?;

        for entry in entries.flatten() {
            let temp_path = entry.path().join("temp");
            if let Ok(contents) = fs::read_to_string(&temp_path)
            && let Ok(raw) = contents.trim().parse::<f64>() {
                // Assume millidegrees Celsius
                return Some(raw / 1000.0);
            }
        }

        None
    }

    let celsius = match read_cpu_temp_celsius() {
        Some(c) => c,
        None => return format.to_string(),
    };

    let kelvin = celsius + 273.15;
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    let c = format!("{:.1}", celsius);
    let k = format!("{:.1}", kelvin);
    let f = format!("{:.1}", fahrenheit);

    format
        .replace("{c}", &c)
        .replace("{k}", &k)
        .replace("{f}", &f)
}
