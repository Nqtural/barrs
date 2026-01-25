use async_trait::async_trait;
use chrono::Local;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::sync::mpsc::Sender;
use crate::config::DateConfig;
use crate::{Module, ModuleOutput};

/// Display date using a configured format
#[derive(Debug)]
pub struct DateModule {
    tx: Sender<()>,
    interval: u64,
    current_date: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl DateModule {
    pub fn new(config: &DateConfig, tx: Sender<()>) -> Self {
        let format = config.format.clone();
        Self {
            tx,
            interval: config.interval,
            current_date: Mutex::new(date_from_string(&format)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

#[async_trait]
impl Module for DateModule {
    async fn run(&self) {
        loop {
            *self.current_date.lock().await = date_from_string(&self.format);
            let _ = self.tx.send(());
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_date.lock().await.clone(),
        }
    }
}

fn date_from_string(format: &str) -> String {
    Local::now().format(format).to_string()
}
