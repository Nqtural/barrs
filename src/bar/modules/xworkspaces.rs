use anyhow::Result;
use std::collections::HashSet;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;
use crate::config::XworkspacesConfig;
use crate::{Module, ModuleOutput};

/// Display X11 workspaces using a configured format
#[derive(Debug)]
pub struct XworkspacesModule {
    current_layout: String,
    icon: Option<String>,
    icon_color: Option<String>,
    format_active: String,
    format_empty: String,
    format_occupied: String,
    format_urgent: String,
    sepparator: String,
}

impl XworkspacesModule {
    pub fn new(config: &XworkspacesConfig) -> Self {
        let format_active = config.format_active.clone();
        let format_empty = config.format_empty.clone();
        let format_occupied = config.format_occupied.clone();
        let format_urgent = config.format_urgent.clone();
        let sepparator = config.sepparator.clone();
        Self {
            current_layout: format_workspaces(
                &format_active,
                &format_empty,
                &format_occupied,
                &format_urgent,
                &sepparator,
            ),
            icon: config.icon.clone(),
            icon_color: config.icon_color.clone(),
            format_active,
            format_empty,
            format_occupied,
            format_urgent,
            sepparator,
        }
    }
}

impl Module for XworkspacesModule {
    fn update(&mut self) {
        self.current_layout = format_workspaces(
            &self.format_active,
            &self.format_empty,
            &self.format_occupied,
            &self.format_urgent,
            &self.sepparator,
        );
    }

    fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: self.icon.clone(),
            icon_color: self.icon_color.clone(),
            value: self.current_layout.clone()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum WorkspaceState {
    Active,
    Empty,
    Occupied,
    Urgent,
    Error,
}

#[derive(Debug)]
struct Workspace {
    pub index: usize,
    pub name: String,
    pub state: WorkspaceState,
}

fn get_workspaces_result_wrapper() -> Vec<Workspace> {
    match get_workspaces() {
        Ok(result) => result,
        Err(e) => vec!(Workspace{
            index: 0,
            name: format!("error: {e}"),
            state: WorkspaceState::Error,
        })
    }
}

fn get_workspaces() -> Result<Vec<Workspace>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let net_number_of_desktops =
        conn.intern_atom(false, b"_NET_NUMBER_OF_DESKTOPS")?.reply()?.atom;
    let net_current_desktop =
        conn.intern_atom(false, b"_NET_CURRENT_DESKTOP")?.reply()?.atom;
    let net_desktop_names =
        conn.intern_atom(false, b"_NET_DESKTOP_NAMES")?.reply()?.atom;
    let net_client_list =
        conn.intern_atom(false, b"_NET_CLIENT_LIST")?.reply()?.atom;
    let net_wm_desktop =
        conn.intern_atom(false, b"_NET_WM_DESKTOP")?.reply()?.atom;
    let net_wm_state =
        conn.intern_atom(false, b"_NET_WM_STATE")?.reply()?.atom;
    let net_wm_state_demands_attention =
        conn.intern_atom(false, b"_NET_WM_STATE_DEMANDS_ATTENTION")?
            .reply()?
            .atom;
    let utf8_string =
        conn.intern_atom(false, b"UTF8_STRING")?.reply()?.atom;

    let num_desktops = conn
        .get_property(false, root, net_number_of_desktops, AtomEnum::CARDINAL, 0, 1)?
        .reply()?
        .value32()
        .and_then(|mut v| v.next())
        .unwrap_or(0) as usize;

    let current_desktop = conn
        .get_property(false, root, net_current_desktop, AtomEnum::CARDINAL, 0, 1)?
        .reply()?
        .value32()
        .and_then(|mut v| v.next())
        .unwrap_or(0) as usize;

    let names_reply = conn
        .get_property(false, root, net_desktop_names, utf8_string, 0, u32::MAX)?
        .reply()?;

    let mut names: Vec<String> = if !names_reply.value.is_empty() {
        String::from_utf8_lossy(&names_reply.value)
            .split('\0')
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    names.resize(num_desktops, String::new());

    let mut occupied = vec![false; num_desktops];
    let mut urgent = HashSet::new();

    let clients_reply = conn
        .get_property(false, root, net_client_list, AtomEnum::WINDOW, 0, u32::MAX)?
        .reply()?;

    if let Some(windows) = clients_reply.value32() {
        for window in windows {
            let desktop_reply = conn
                .get_property(false, window, net_wm_desktop, AtomEnum::CARDINAL, 0, 1)?
                .reply()?;

            let desktop = match desktop_reply.value32().and_then(|mut v| v.next()) {
                Some(d) if (d as usize) < num_desktops => d as usize,
                _ => continue,
            };

            occupied[desktop] = true;

            let state_reply = conn
                .get_property(false, window, net_wm_state, AtomEnum::ATOM, 0, u32::MAX)?
                .reply()?;

            if let Some(mut states) = state_reply.value32()
            && states.any(|a| a == net_wm_state_demands_attention) {
                urgent.insert(desktop);
            }
        }
    }

    let mut result = Vec::with_capacity(num_desktops);

    for i in 0..num_desktops {
        let state = if i == current_desktop {
            WorkspaceState::Active
        } else if urgent.contains(&i) {
            WorkspaceState::Urgent
        } else if occupied[i] {
            WorkspaceState::Occupied
        } else {
            WorkspaceState::Empty
        };

        result.push(Workspace {
            index: i,
            name: names[i].clone(),
            state,
        });
    }

    Ok(result)
}

fn format_workspaces(
    format_active: &str,
    format_empty: &str,
    format_occupied: &str,
    format_urgent: &str,
    separator: &str,
) -> String {
    let workspaces = get_workspaces_result_wrapper();
    let mut parts = Vec::with_capacity(workspaces.len());

    for ws in workspaces {
        let template = match ws.state {
            WorkspaceState::Active => format_active,
            WorkspaceState::Empty => format_empty,
            WorkspaceState::Occupied => format_occupied,
            WorkspaceState::Urgent => format_urgent,
            WorkspaceState::Error => "",
        };

        let formatted = template
            .replace("{index}", &ws.index.to_string())
            .replace("{name}", &ws.name);

        parts.push(formatted);
    }

    parts.join(separator)
}
