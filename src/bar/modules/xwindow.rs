use async_trait::async_trait;
use tokio::sync::Mutex;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;
use crate::config::XwindowConfig;
use crate::{Module, ModuleOutput};

/// Display current window name on X11
#[derive(Debug)]
pub struct XwindowModule {
    signal_id: Option<u8>,
    current_window: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
    max_length: u32,
    user_empty_string: String,
}

impl XwindowModule {
    pub fn new(config: &XwindowConfig) -> Self {
        let max_length = config.max_length;
        let user_empty_string = config.empty_name.clone();
        Self {
            signal_id: config.signal_id,
            current_window: Mutex::new(get_active_window_title(max_length, &user_empty_string)),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            max_length,
            user_empty_string,
        }
    }
}

#[async_trait]
impl Module for XwindowModule {
    fn signal_id(&self) -> Option<u8> {
        self.signal_id
    }

    async fn run(&self) {
        *self.current_window.lock().await = get_active_window_title(self.max_length, &self.user_empty_string);
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_window.lock().await.clone()
        }
    }
}

fn get_active_window_title(max_length: u32, user_empty_string: &str) -> String {
    let (conn, screen_num) = match RustConnection::connect(None) {
        Ok(result) => result,
        Err(e) => return format!("error: {e}"),
    };

    let root = conn.setup().roots[screen_num].root;

    let net_active_window_atom = match conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .unwrap()
        .reply()
    {
        Ok(r) => r.atom,
        Err(_) => return "Unknown window".to_string(),
    };

    let active_window_id = match conn
        .get_property(false, root, net_active_window_atom, AtomEnum::WINDOW, 0, 1)
        .unwrap()
        .reply()
    {
        Ok(prop) => prop
            .value32()
            .and_then(|mut iter| iter.next())
            .unwrap_or(0),
        Err(_) => 0,
    };

    if active_window_id == 0 {
        return user_empty_string.to_string();
    }

    let net_wm_name = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .unwrap()
        .reply()
        .ok()
        .map(|r| r.atom);

    let wm_name = conn
        .intern_atom(false, b"WM_NAME")
        .unwrap()
        .reply()
        .ok()
        .map(|r| r.atom);

    let utf8_string = conn
        .intern_atom(false, b"UTF8_STRING")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    let compound_text = conn
        .intern_atom(false, b"COMPOUND_TEXT")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    // 1. _NET_WM_NAME (UTF8_STRING)
    if let Some(atom) = net_wm_name
    && let Ok(reply) = conn
        .get_property(false, active_window_id, atom, utf8_string, 0, max_length)
        .unwrap()
        .reply()
    && !reply.value.is_empty() {
        return String::from_utf8_lossy(&reply.value).to_string();
    }

    // 2. WM_NAME (COMPOUND_TEXT)
    if let Some(atom) = wm_name
    && let Ok(reply) = conn
        .get_property(false, active_window_id, atom, compound_text, 0, max_length)
        .unwrap()
        .reply()
    && !reply.value.is_empty() {
        // Best-effort decode; spec-correct decoding requires Xlib helpers
        return String::from_utf8_lossy(&reply.value).to_string();
    }

    // 3. WM_NAME (STRING, Latin-1)
    if let Some(atom) = wm_name
    && let Ok(reply) = conn
        .get_property(false, active_window_id, atom, AtomEnum::STRING, 0, max_length)
        .unwrap()
        .reply()
    && !reply.value.is_empty() {
        return reply.value.iter().map(|&b| b as char).collect();
    }

    "Unknown window".to_string()
}
