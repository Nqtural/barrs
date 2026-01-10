use async_trait::async_trait;
use std::fs;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use crate::config::BatteryConfig;
use crate::{Module, ModuleOutput};

/// Display battery info using a configured format
#[derive(Debug)]
pub struct BatteryModule {
    interval: u64,
    current_battery: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    name: String,
    format_charging: String,
    format_discharging: String,
    format_full: String,
}

impl BatteryModule {
    pub fn new(config: &BatteryConfig) -> Self {
        let interval = config.interval;
        let name = config.name.clone();
        let format_charging = config.format_charging.clone();
        let format_discharging = config.format_discharging.clone();
        let format_full = config.format_full.clone();
        Self {
            interval,
            current_battery: Mutex::new(battery_from_string(
                &name,
                &format_charging,
                &format_discharging,
                &format_full,
            )),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            name,
            format_charging,
            format_discharging,
            format_full,
        }
    }
}

#[async_trait]
impl Module for BatteryModule {
    async fn run(&self) {
        loop {
            {
                *self.current_battery.lock().await = battery_from_string(
                    &self.name,
                    &self.format_charging,
                    &self.format_discharging,
                    &self.format_full,
                );
            }
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_battery.lock().await.clone(),
        }
    }
}

fn read_battery_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn battery_from_string(
    name: &str,
    format_charging: &str,
    format_discharging: &str,
    format_full: &str,
) -> String {
    let base = format!("/sys/class/power_supply/{name}");

    let status = read_battery_file(&format!("{base}/status")).unwrap_or_else(|| "Unknown".into());
    let capacity = read_battery_file(&format!("{base}/capacity")).unwrap_or_else(|| "0".into());

    let energy_now = read_battery_file(&format!("{base}/energy_now"))
        .and_then(|v| v.parse::<f64>().ok());
    let energy_full = read_battery_file(&format!("{base}/energy_full"))
        .and_then(|v| v.parse::<f64>().ok());
    let power_now = read_battery_file(&format!("{base}/power_now"))
        .and_then(|v| v.parse::<f64>().ok());

    let (current, full, draw) = if let (Some(e_now), Some(e_full), Some(p_now)) =
        (energy_now, energy_full, power_now)
    {
        (e_now, e_full, p_now)
    } else {
        let c_now = read_battery_file(&format!("{base}/charge_now"))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let c_full = read_battery_file(&format!("{base}/charge_full"))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(1.0); // avoid division by 0
        let c_draw = read_battery_file(&format!("{base}/current_now"))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(1.0);

        (c_now, c_full, c_draw)
    };

    let time_str = if draw > 0.0 {
        let hours_remaining = match status.as_str() {
            "Discharging" => current / draw,
            "Charging" => (full - current) / draw,
            _ => 0.0,
        };

        let hours = hours_remaining.floor() as u32;
        let minutes = ((hours_remaining - hours as f64) * 60.0).round() as u32;
        format!("{:02}:{:02}", hours, minutes)
    } else {
        "--:--".to_string()
    };

    let template = match status.as_str() {
        "Charging" => format_charging,
        "Discharging" => format_discharging,
        "Not charging" => format_discharging,
        "Full" => format_full,
        _ => "error: unknown battery status: {status}",
    };

    template
        .replace("{%}", &capacity)
        .replace("{status}", &status)
        .replace("{time}", &time_str)
}
