use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::config::CpuConfig;
use crate::{Module, ModuleOutput};

#[derive(Debug)]
pub struct CpuModule {
    interval: u64,
    current_usage: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
    prev_total: AtomicU64,
    prev_idle: AtomicU64,
}

impl CpuModule {
    pub fn new(config: &CpuConfig) -> Self {
        let format = config.format.clone();
        let (total, idle) = read_cpu_jiffies().unwrap_or((0, 0));

        Self {
            interval: config.interval,
            current_usage: Mutex::new(calculate_usage(&format, total, idle, total, idle)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
            prev_total: AtomicU64::new(total),
            prev_idle: AtomicU64::new(idle),
        }
    }
}

#[async_trait]
impl Module for CpuModule {
    async fn run(&self) {
        loop {
            if let Some((total, idle)) = read_cpu_jiffies() {
                *self.current_usage.lock().await = calculate_usage(
                    &self.format,
                    self.prev_total.load(Ordering::SeqCst),
                    self.prev_idle.load(Ordering::SeqCst),
                    total,
                    idle,
                );

                // Save snapshot for next update
                self.prev_total.store(total, Ordering::SeqCst);
                self.prev_idle.store(idle, Ordering::SeqCst);
            }
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_usage.lock().await.clone(),
        }
    }
}

fn read_cpu_jiffies() -> Option<(u64, u64)> {
    let stat = fs::read_to_string("/proc/stat").ok()?;
    for line in stat.lines() {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 { return None; }

            let user: u64 = parts[1].parse().ok()?;
            let nice: u64 = parts[2].parse().ok()?;
            let system: u64 = parts[3].parse().ok()?;
            let idle: u64 = parts[4].parse().ok()?;
            let iowait: u64 = if parts.len() > 5 { parts[5].parse().unwrap_or(0) } else { 0 };

            let idle_total = idle + iowait;
            let total = user + nice + system + idle_total;
            return Some((total, idle_total));
        }
    }
    None
}

fn calculate_usage(
    format: &str,
    prev_total: u64,
    prev_idle: u64,
    total: u64,
    idle: u64,
) -> String {
    let total_delta = total.saturating_sub(prev_total);
    let idle_delta = idle.saturating_sub(prev_idle);
    let usage = if total_delta == 0 {
        0.0
    } else {
        ((total_delta - idle_delta) as f64 / total_delta as f64) * 100.0
    };

    format.replace("{%}", &format!("{:.1}", usage))
}
