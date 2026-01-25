use async_trait::async_trait;
use if_addrs::get_if_addrs;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use crate::config::NetworkConfig;
use crate::{Module, ModuleOutput};

/// Display information about a given network interface using a configured format
#[derive(Debug)]
pub struct NetworkModule {
    tx: Sender<()>,
    interval: u64,
    current_net: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    interface: String,
    format: String,
    prev_rx: AtomicU64,
    prev_tx: AtomicU64,
}

impl NetworkModule {
    pub fn new(config: &NetworkConfig, tx: Sender<()>) -> Self {
        let interval = config.interval;
        let interface = config.interface.clone();
        let format = config.format.clone();
        let (current_net, prev_rx, prev_tx) = network_info_from_string(
            &interface,
            &format,
            0,
            0,
            interval,
        );

        Self {
            tx,
            interval,
            current_net: Mutex::new(current_net),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            interface,
            format,
            prev_rx: AtomicU64::new(prev_rx),
            prev_tx: AtomicU64::new(prev_tx),
        }
    }
}

#[async_trait]
impl Module for NetworkModule {
    async fn run(&self) {
        loop {
            let (current_net, prev_rx, prev_tx) = network_info_from_string(
                &self.interface,
                &self.format,
                self.prev_rx.load(Ordering::SeqCst),
                self.prev_tx.load(Ordering::SeqCst),
                self.interval,
            );
            *self.current_net.lock().await = current_net;
            self.prev_rx.store(prev_rx, Ordering::SeqCst);
            self.prev_tx.store(prev_tx, Ordering::SeqCst);

            let _ = self.tx.send(());
            sleep(Duration::from_secs(self.interval)).await;
        }
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_net.lock().await.clone(),
        }
    }
}

/// Read a file from sysfs and trim it
fn read_sysfs_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

/// Read total RX and TX bytes for the interface
fn read_bytes(iface: &str) -> (u64, u64) {
    let base = format!("/sys/class/net/{iface}");
    let rx = read_sysfs_file(&format!("{base}/statistics/rx_bytes"))
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let tx = read_sysfs_file(&format!("{base}/statistics/tx_bytes"))
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    (rx, tx)
}

/// Read interface state
fn read_state(iface: &str) -> String {
    read_sysfs_file(&format!("/sys/class/net/{iface}/operstate"))
        .unwrap_or_else(|| "unknown".into())
}

/// Convert bytes/sec to different units
fn bytes_to_units(bytes_per_sec: u64) -> (u64, f64, f64, f64) {
    let bit = bytes_per_sec * 8;
    let kbit = bit as f64 / 1_000.0;
    let mbit = bit as f64 / 1_000_000.0;
    let gbit = bit as f64 / 1_000_000_000.0;
    (bit, kbit, mbit, gbit)
}

/// Parse IPv4 from `/proc/net/fib_trie` for the given interface
fn get_ipv4_address(iface: &str) -> Option<String> {
    let addrs = get_if_addrs().ok()?;
    for addr in addrs {
        if addr.name == iface
        && let std::net::IpAddr::V4(ipv4) = addr.ip() {
            return Some(ipv4.to_string());
        }
    }

    None
}

/// Replace all placeholders in the format string
pub fn network_info_from_string(
    iface: &str,
    format: &str,
    prev_rx: u64,
    prev_tx: u64,
    delta_secs: u64,
) -> (String, u64, u64) {
    let state = read_state(iface);
    let (rx_bytes, tx_bytes) = read_bytes(iface);

    let rx_speed_bps = rx_bytes.saturating_sub(prev_rx) / delta_secs;
    let tx_speed_bps = tx_bytes.saturating_sub(prev_tx) / delta_secs;

    let (rx_bit, rx_kbit, rx_mbit, rx_gbit) = bytes_to_units(rx_speed_bps);
    let (tx_bit, tx_kbit, tx_mbit, tx_gbit) = bytes_to_units(tx_speed_bps);

    let formatted = format
        .replace("{ip}", &get_ipv4_address(iface).unwrap_or(String::from("unknown")))
        .replace("{iface}", iface)
        .replace("{state}", &state)
        .replace("{down_speed_bit}", &rx_bit.to_string())
        .replace("{down_speed_kbit}", &format!("{:.2}", rx_kbit))
        .replace("{down_speed_mbit}", &format!("{:.2}", rx_mbit))
        .replace("{down_speed_gbit}", &format!("{:.2}", rx_gbit))
        .replace("{up_speed_bit}", &tx_bit.to_string())
        .replace("{up_speed_kbit}", &format!("{:.2}", tx_kbit))
        .replace("{up_speed_mbit}", &format!("{:.2}", tx_mbit))
        .replace("{up_speed_gbit}", &format!("{:.2}", tx_gbit));

    (formatted, rx_bytes, tx_bytes)
}
