use async_trait::async_trait;
use tokio::sync::Mutex;
use x11rb::connection::Connection;
use x11rb::protocol::xproto;
use x11rb::rust_connection::RustConnection;
use crate::config::XkeyboardConfig;
use crate::{Module, ModuleOutput};

/// Display current keyboard layout on X11
#[derive(Debug)]
pub struct XkeyboardModule {
    signal_id: Option<u8>,
    current_layout: Mutex<String>,
    icon: Option<String>,
    icon_color: Option<String>,
}

impl XkeyboardModule {
    pub fn new(config: &XkeyboardConfig) -> Self {
        Self {
            signal_id: config.signal_id,
            current_layout: Mutex::new(get_current_keyboard_layout()),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
        }
    }
}

#[async_trait]
impl Module for XkeyboardModule {
    fn signal_id(&self) -> Option<u8> {
        self.signal_id
    }

    async fn run(&self) {
        *self.current_layout.lock().await = get_current_keyboard_layout();
    }

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_layout.lock().await.clone()
        }
    }
}

fn get_current_keyboard_layout() -> String {
    // Connect to the X server
    let (conn, screen_num) = match RustConnection::connect(None) {
        Ok(r) => r,
        Err(e) => return format!("error: {e}"),
    };
    let root = conn.setup().roots[screen_num].root;

    // Intern the atom for "_XKB_RULES_NAMES"
    let atom = match xproto::intern_atom(&conn, false, b"_XKB_RULES_NAMES") {
        Ok(cookie) => match cookie.reply() {
            Ok(atom_reply) => atom_reply.atom,
            Err(_) => return "unknown".to_string(),
        },
        Err(_) => return "unknown".to_string(),
    };

    // Get the property data from the root window
    let prop = match xproto::get_property(
        &conn,
        false,
        root,
        atom,
        xproto::AtomEnum::STRING,
        0,
        u32::MAX,
    ) {
        Ok(cookie) => match cookie.reply() {
            Ok(prop_reply) => prop_reply,
            Err(_) => return "unknown".to_string(),
        },
        Err(_) => return "unknown".to_string(),
    };

    // Convert to a Rust String
    let text = String::from_utf8_lossy(&prop.value).to_string();

    // The property value is NUL-separated: rules\0model\0layout\0variant\0options\0
    let parts: Vec<&str> = text.split('\0').collect();
    if parts.len() > 2 && !parts[2].is_empty() {
        parts[2].to_string()
    } else {
        "unknown".to_string()
    }
}
