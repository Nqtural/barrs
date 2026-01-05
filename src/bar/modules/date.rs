use chrono::Local;
use crate::config::DateConfig;
use crate::{Module, ModuleOutput};

/// Display date using a configured format
#[derive(Debug)]
pub struct DateModule {
    current_date: String,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
}

impl DateModule {
    pub fn new(config: &DateConfig) -> Self {
        let format = config.format.clone();
        Self {
            current_date: date_from_string(&format),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
        }
    }
}

impl Module for DateModule {
    fn update(&mut self) {
        self.current_date = date_from_string(&self.format);
    }

    fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_date.clone(),
        }
    }
}

fn date_from_string(format: &str) -> String {
    Local::now().format(format).to_string()
}
