use crate::config::Config;
use crate::{
    build_modules,
    Module,
};

pub struct Bar {
    left: Vec<Box<dyn Module>>,
    center: Vec<Box<dyn Module>>,
    right: Vec<Box<dyn Module>>,

    sepparator: String,
}

impl Bar {
    pub fn new(config: &Config) -> Self {
        Self {
            left: build_modules(&config.left, &config.modules),
            center: build_modules(&config.center, &config.modules),
            right: build_modules(&config.right, &config.modules),
            sepparator: config.sepparator.clone(),
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
        let left = self.left
            .iter()
            .map(|m| m.get_value())
            .collect::<Vec<&str>>()
            .join(&self.sepparator);
        let center = self.center
            .iter()
            .map(|m| m.get_value())
            .collect::<Vec<&str>>()
            .join(&self.sepparator);
        let right = self.right
            .iter()
            .map(|m| m.get_value())
            .collect::<Vec<&str>>()
            .join(&self.sepparator);

        [left, center, right].join("   ")
    }
}
