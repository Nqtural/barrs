mod date;
use date::DateModule;

mod invalid;
use invalid::InvalidModule;

mod loadavg;
use loadavg::LoadavgModule;

mod factory;
pub use factory::build_modules;
