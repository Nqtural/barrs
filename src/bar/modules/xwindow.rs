use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;
use crate::config::XwindowConfig;
use crate::{Module, ModuleOutput};

/// Display current window name on X11
#[derive(Debug)]
pub struct XwindowModule {
    current_window: String,
    icon: Option<String>,
    icon_color: Option<String>,
    max_length: u32,
}

impl XwindowModule {
    pub fn new(config: &XwindowConfig) -> Self {
        let max_length = config.max_length;
        Self {
            current_window: get_active_window_title(max_length),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            max_length,
        }
    }
}

impl Module for XwindowModule {
    fn update(&mut self) {
        self.current_window = get_active_window_title(self.max_length);
    }

    fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_window.clone()
        }
    }
}

fn get_active_window_title(max_length: u32) -> String {
    let (conn, screen_num) = match RustConnection::connect(None) {
        Ok(result) => result,
        Err(e) => return format!("error: {e}"),
    };
    let root = conn.setup().roots[screen_num].root;

    let net_active_window_atom = match conn.intern_atom(false, b"_NET_ACTIVE_WINDOW").unwrap().reply() {
        Ok(r) => r.atom,
        Err(_) => return "Unknown window".to_string(),
    };

    let active_window_id = match conn.get_property(
        false,
        root,
        net_active_window_atom,
        AtomEnum::WINDOW,
        0,
        1,
    ).unwrap().reply() {
        Ok(prop) => prop.value32().and_then(|mut iter| iter.next()).unwrap_or(0),
        Err(_) => 0,
    };

    if active_window_id == 0 {
        return "No active window".to_string();
    }

    let net_wm_name_atom = conn.intern_atom(false, b"_NET_WM_NAME").unwrap().reply().ok().map(|r| r.atom);
    let wm_name_atom = conn.intern_atom(false, b"WM_NAME").unwrap().reply().ok().map(|r| r.atom);

    // Try _NET_WM_NAME first (UTF-8)
    if let Some(atom) = net_wm_name_atom
    && let Ok(reply) = conn.get_property(false, active_window_id, atom, AtomEnum::STRING, 0, max_length).unwrap().reply()
    && !reply.value.is_empty() {
        return String::from_utf8_lossy(&reply.value).to_string();
    }

    // Fallback to WM_NAME
    if let Some(atom) = wm_name_atom
    && let Ok(reply) = conn.get_property(false, active_window_id, atom, AtomEnum::STRING, 0, max_length).unwrap().reply()
    && !reply.value.is_empty() {
        // Some older apps use Latin-1 instead of UTF-8
        return reply.value.iter().map(|&b| b as char).collect();
    }

    "Unknown window".to_string()
}
