use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use std::ffi::CString;
use libc;
use std::mem::MaybeUninit;
use crate::config::FilesystemConfig;
use crate::{Module, ModuleOutput};

/// Display information about the filesystem using a configured format
#[derive(Debug)]
pub struct FilesystemModule {
    interval: u64,
    current_fs_info: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    format: String,
    mountpoint: String,
}

impl FilesystemModule {
    pub fn new(config: &FilesystemConfig) -> Self {
        let format = config.format.clone();
        let mountpoint = config.mountpoint.clone();
        Self {
            interval: config.interval,
            current_fs_info: Mutex::new(fs_info_from_string(&format, &mountpoint)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format,
            mountpoint,
        }
    }
}

#[async_trait]
impl Module for FilesystemModule {
    async fn run(&self) {
        loop {
            *self.current_fs_info.lock().await = fs_info_from_string(&self.format, &self.mountpoint);
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_fs_info.lock().await.clone(),
        }
    }
}

struct FsStats {
    total: u64,
    free: u64,
    used: u64,
}

fn fs_info_from_string(format: &str, mountpoint: &str) -> String {
    let stats = match read_fs_stats(mountpoint) {
        Some(s) => s,
        None => return format.replace("{mount}", mountpoint),
    };

    let used_pct = if stats.total > 0 {
        stats.used as f64 / stats.total as f64 * 100.0
    } else {
        0.0
    };

    let free_pct = 100.0 - used_pct;

    format
        .replace("{mount}", mountpoint)

        // IEC totals
        .replace("{kib_total}", &kib(stats.total).to_string())
        .replace("{mib_total}", &mib(stats.total).to_string())
        .replace("{gib_total}", &gib(stats.total).to_string())
        .replace("{tib_total}", &tib(stats.total).to_string())

        // IEC free
        .replace("{kib_free}", &kib(stats.free).to_string())
        .replace("{mib_free}", &mib(stats.free).to_string())
        .replace("{gib_free}", &gib(stats.free).to_string())
        .replace("{tib_free}", &tib(stats.free).to_string())

        // IEC used
        .replace("{kib_used}", &kib(stats.used).to_string())
        .replace("{mib_used}", &mib(stats.used).to_string())
        .replace("{gib_used}", &gib(stats.used).to_string())
        .replace("{tib_used}", &tib(stats.used).to_string())

        // SI totals
        .replace("{kb_total}", &kb(stats.total).to_string())
        .replace("{mb_total}", &mb(stats.total).to_string())
        .replace("{gb_total}", &gb(stats.total).to_string())
        .replace("{tb_total}", &tb(stats.total).to_string())

        // SI free
        .replace("{kb_free}", &kb(stats.free).to_string())
        .replace("{mb_free}", &mb(stats.free).to_string())
        .replace("{gb_free}", &gb(stats.free).to_string())
        .replace("{tb_free}", &tb(stats.free).to_string())

        // SI used
        .replace("{kb_used}", &kb(stats.used).to_string())
        .replace("{mb_used}", &mb(stats.used).to_string())
        .replace("{gb_used}", &gb(stats.used).to_string())
        .replace("{tb_used}", &tb(stats.used).to_string())

        // Percentages
        .replace("{%_used}", &format!("{:.1}", used_pct))
        .replace("{%_free}", &format!("{:.1}", free_pct))
}

fn kib(b: u64) -> u64 { b / 1024 }
fn mib(b: u64) -> u64 { b / 1024 / 1024 }
fn gib(b: u64) -> u64 { b / 1024 / 1024 / 1024 }
fn tib(b: u64) -> u64 { b / 1024 / 1024 / 1024 / 1024 }

fn kb(b: u64) -> u64 { b / 1000 }
fn mb(b: u64) -> u64 { b / 1000 / 1000 }
fn gb(b: u64) -> u64 { b / 1000 / 1000 / 1000 }
fn tb(b: u64) -> u64 { b / 1000 / 1000 / 1000 / 1000 }

fn read_fs_stats(mountpoint: &str) -> Option<FsStats> {
    let c_path = CString::new(mountpoint).ok()?;

    let mut stat = MaybeUninit::<libc::statvfs>::uninit();
    let res = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };

    if res != 0 {
        return None;
    }

    let stat = unsafe { stat.assume_init() };

    let block_size = stat.f_frsize;
    let total = stat.f_blocks * block_size;
    let free = stat.f_bavail * block_size;
    let used = total.saturating_sub(free);

    Some(FsStats { total, free, used })
}
