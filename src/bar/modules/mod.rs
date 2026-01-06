mod battery;
use battery::BatteryModule;

mod brightnessctl;
use brightnessctl::BrightnessctlModule;

mod date;
use date::DateModule;

mod filesystem;
use filesystem::FilesystemModule;

mod invalid;
use invalid::InvalidModule;

mod loadavg;
use loadavg::LoadavgModule;

mod memory;
use memory::MemoryModule;

mod network;
use network::NetworkModule;

mod wpctl;
use wpctl::WpctlModule;

mod xkeyboard;
use xkeyboard::XkeyboardModule;

mod xwindow;
use xwindow::XwindowModule;

mod xworkspaces;
use xworkspaces::XworkspacesModule;

mod factory;
pub use factory::build_modules;
