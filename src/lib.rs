mod modules;
use modules::build_modules;

mod module_trait;
pub use module_trait::Module;

mod bar;
pub use bar::Bar;

pub mod config;
