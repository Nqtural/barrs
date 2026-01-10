use async_trait::async_trait;
use crate::{Module, ModuleOutput};

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

#[async_trait]
impl Module for InvalidModule {
    async fn run(&self) {}

    async fn get_value(&self) -> ModuleOutput {
        ModuleOutput {
            icon: None,
            icon_color: None,
            value: self.value.clone(),
        }
    }
}
