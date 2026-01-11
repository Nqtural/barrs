use async_trait::async_trait;
use tokio::sync::Mutex;
use std::process::Command;
use crate::config::WpctlConfig;
use crate::{Module, ModuleOutput};

/// Display wpctl info using a configured format
#[derive(Debug)]
pub struct WpctlModule {
    signal_id: Option<u8>,
    current_audio: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
    format_muted: String,
}

impl WpctlModule {
    pub fn new(config: &WpctlConfig) -> Self {
        let format = config.format.clone();
        let format_muted = config.format_muted.clone();
        Self {
            signal_id: config.signal_id,
            current_audio: Mutex::new(audio_from_string(&format, &format_muted)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
            format_muted,
        }
    }
}

#[async_trait]
impl Module for WpctlModule {
    fn signal_id(&self) -> Option<u8> {
        self.signal_id
    }

    async fn run(&self) {
        *self.current_audio.lock().await = audio_from_string(&self.format, &self.format_muted);
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_audio.lock().await.clone(),
        }
    }
}

fn audio_from_string(format: &str, format_muted: &str) -> String {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output();

    let output = match output {
        Ok(out) => out,
        Err(_) => return "error: failed to execute wpctl".to_string(),
    };

    if !output.status.success() {
        return format!(
            "error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error"),
        );
    }

    let stdout = str::from_utf8(&output.stdout).unwrap_or("");

    let mut volume: Option<f32> = None;
    let mut is_muted = false;

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Volume:") {
            if let Some(v_str) = trimmed.split_whitespace().nth(1) {
                volume = v_str.parse::<f32>().ok();
            }
            if trimmed.contains("[MUTED]") {
                is_muted = true;
            }
        }
    }

    let volume_percent = volume.unwrap_or(0.0) * 100.0;

    if is_muted {
        format_muted.replace("{volume}", &format!("{:.0}", volume_percent))
    } else {
        format.replace("{volume}", &format!("{:.0}", volume_percent))
    }
}
