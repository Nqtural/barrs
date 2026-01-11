use async_trait::async_trait;
use super::ModuleOutput;

#[async_trait]
pub trait Module: Send + Sync {
    async fn run(&self);
    async fn get_value(&self) -> ModuleOutput;

    fn signal_id(&self) -> Option<u8> {
        None
    }
}
