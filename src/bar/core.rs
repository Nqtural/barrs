use crate::Config;
use crate::{Module, ModuleOutput};
use super::build_modules;

pub struct Bar {
    left: Vec<Box<dyn Module>>,
    center: Vec<Box<dyn Module>>,
    right: Vec<Box<dyn Module>>,

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

    pub fn update(&mut self) {
        [&mut self.left, &mut self.center, &mut self.right]
            .iter_mut()
                .for_each(|ml| {
                    ml.iter_mut().for_each(|m| m.update());
                });
    }

    pub fn construct(&self) -> String {
        match self.frontend.as_str() {
            "lemonbar" => self.construct_lemonbar(),
            _ => {
                eprintln!("warning: frontend {} not implemented", &self.frontend);
                self.construct_generic()
            }
        }
    }

    fn collect_sections(
        &self,
    ) -> (Vec<ModuleOutput>, Vec<ModuleOutput>, Vec<ModuleOutput>) {
        (
            self.left
                .iter()
                .map(|m| m.get_value())
                .collect::<Vec<ModuleOutput>>(),
            self.center
                .iter()
                .map(|m| m.get_value())
                .collect::<Vec<ModuleOutput>>(),
            self.right
                .iter()
                .map(|m| m.get_value())
                .collect::<Vec<ModuleOutput>>(),
        )
    }

    fn construct_lemonbar_module(&self, m: &ModuleOutput) -> String {
        match (&m.icon, &m.icon_color) {
            (Some(icon), Some(color)) => {
                format!("%{{F{}}}{}%{{F-}}{}", color, icon, m.value)
            }
            (Some(icon), None) => {
                format!("{}{}", icon, m.value)
            }
            (None, _) => {
                m.value.clone()
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

    fn construct_lemonbar(&self) -> String {
        let (left, center, right) = self.collect_sections();

        format!(
            // alignement in lemonbar is done with %{l}, %{c} and %{r}
            "%{{l}}{}%{{c}}{}%{{r}}{}",
            self.construct_lemonbar_section(left),
            self.construct_lemonbar_section(center),
            self.construct_lemonbar_section(right),
        )
    }

    fn construct_generic(&self) -> String {
        let (left, center, right) = self.collect_sections();
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
