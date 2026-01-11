use std::sync::Arc;
use crate::Config;
use crate::{Module, ModuleOutput};
use super::build_modules;

pub struct Bar {
    left: Vec<Arc<dyn Module + Sync + Send>>,
    center: Vec<Arc<dyn Module + Sync + Send>>,
    right: Vec<Arc<dyn Module + Sync + Send>>,

    separator: String,
    frontend: String,
}

impl Bar {
    pub fn new(config: &Config) -> Self {
        Self {
            left: build_modules(&config.left, &config.modules),
            center: build_modules(&config.center, &config.modules),
            right: build_modules(&config.right, &config.modules),
            separator: config.separator.clone(),
            frontend: config.frontend.clone(),
        }
    }

    pub async fn construct(&self) -> String {
        match self.frontend.as_str() {
            "lemonbar" => self.construct_lemonbar().await,
            _ => {
                eprintln!("warning: frontend {} not implemented", &self.frontend);
                self.construct_generic().await
            }
        }
    }

    pub fn start_modules(&self) {
        for module in self.left.iter().chain(self.center.iter()).chain(self.right.iter()) {
            let module_clone = Arc::clone(module);
            tokio::spawn(async move {
                module_clone.run().await;
            });
        }
    }

    async fn collect_sections(
        &self,
    ) -> (Vec<ModuleOutput>, Vec<ModuleOutput>, Vec<ModuleOutput>) {
        async fn collect(modules: &[Arc<dyn Module + Send + Sync>]) -> Vec<ModuleOutput> {
            let mut results = Vec::with_capacity(modules.len());
            for m in modules {
                results.push(m.get_value().await);
            }
            results
        }

        let left = collect(&self.left).await;
        let center = collect(&self.center).await;
        let right = collect(&self.right).await;

        (left, center, right)
    }

    fn construct_lemonbar_module(&self, m: &ModuleOutput) -> String {
        let safe_value = m.value.replace('%', "%%");
        match (&m.icon, &m.icon_color) {
            (Some(icon), Some(color)) => {
                let safe_icon = icon.replace('%', "%%");
                format!("%{{F{}}}{}%{{F-}}{}", color, safe_icon, safe_value)
            }
            (Some(icon), None) => {
                let safe_icon = icon.replace('%', "%%");
                format!("{}{}", safe_icon, safe_value)
            }
            (None, _) => {
                safe_value.clone()
            }
        }
    }

    fn construct_lemonbar_section(&self, section: Vec<ModuleOutput>) -> String {
        section
            .iter()
            .map(|m| self.construct_lemonbar_module(m))
            .collect::<Vec<String>>()
            .join(&self.separator)
    }

    async fn construct_lemonbar(&self) -> String {
        let (left, center, right) = self.collect_sections().await;

        format!(
            // alignement in lemonbar is done with %{l}, %{c} and %{r}
            "%{{l}}{}%{{c}}{}%{{r}}{}",
            self.construct_lemonbar_section(left),
            self.construct_lemonbar_section(center),
            self.construct_lemonbar_section(right),
        )
    }

    async fn construct_generic(&self) -> String {
        let (left, center, right) = self.collect_sections().await;
        [
            left
                .iter()
                .map(|m| m.value.clone())
                .collect::<Vec<String>>()
                .join(&self.separator),
            center
                .iter()
                .map(|m| m.value.clone())
                .collect::<Vec<String>>()
                .join(&self.separator),
            right
                .iter()
                .map(|m| m.value.clone())
                .collect::<Vec<String>>()
                .join(&self.separator),
        ].join("   ")
    }
}
