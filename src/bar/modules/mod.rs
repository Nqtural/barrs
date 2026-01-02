mod date;
use date::DateModule;

mod invalid;
use invalid::InvalidModule;

mod loadavg;
use loadavg::LoadavgModule;

mod memory;
use memory::MemoryModule;

mod wpctl;
use wpctl::WpctlModule;

mod factory;
pub use factory::build_modules;
