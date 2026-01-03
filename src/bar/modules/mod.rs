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

mod wpctl;
use wpctl::WpctlModule;

mod xworkspaces;
use xworkspaces::XworkspacesModule;

mod factory;
pub use factory::build_modules;
