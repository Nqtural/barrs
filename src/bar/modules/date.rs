use chrono::Local;
use crate::config::DateConfig;
use crate::Module;

/// Display date using a configured format
#[derive(Debug)]
pub struct DateModule {
    current_date: String,
    format: String,
}

impl DateModule {
    pub fn new(config: &DateConfig) -> Self {
        let format = config.format.clone();
        Self {
            current_date: date_from_string(&format),
            format,
        }
    }
}

impl Module for DateModule {
    fn update(&mut self) {
        self.current_date = date_from_string(&self.format);
    }

    fn get_value(&self) -> &str {
        &self.current_date
    }
}

fn date_from_string(format: &str) -> String {
    Local::now().format(format).to_string()
}
