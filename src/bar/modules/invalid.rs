use crate::Module;

#[derive(Debug)]
pub struct InvalidModule {
    value: String,
}

impl InvalidModule {
    pub fn new(name: &str) -> Self {
        Self {
            value: format!("error: invalid module name: {}", name),
        }
    }
}

impl Module for InvalidModule {
    fn update(&mut self) {}

    fn get_value(&self) -> &str {
        &self.value
    }
}
