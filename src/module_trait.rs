use super::ModuleOutput;

pub trait Module {
    fn update(&mut self);
    fn get_value(&self) -> ModuleOutput;
}
