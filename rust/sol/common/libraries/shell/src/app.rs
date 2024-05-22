// Copyright Ian Stewart 2024, All Rights Reserved.

use std::path::Path;

use sol_runtime::modules::ModuleRegistry;

use crate::modules::ModuleLoader;

#[derive(Debug)]
pub struct App {
    registry: ModuleRegistry,
    module_loaders: Vec<ModuleLoader>,
}

impl App {
    pub fn new(module_paths: &Vec<&Path>) -> Self {
        let mut instance = Self {
            registry: ModuleRegistry::default(),
            module_loaders: Vec::with_capacity(module_paths.len()),
        };

        for module_path in module_paths {}

        instance
    }

    pub fn run(&mut self) {}
}
