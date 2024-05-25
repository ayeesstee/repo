// Copyright Ian Stewart 2024, All Rights Reserved.

use std::path::PathBuf;

use sol_runtime::modules::ModuleRegistry;

use crate::modules::{ModuleCachePath, ModuleLoader, ModuleSourcePath};

#[derive(Debug, Default)]
pub struct App {
    registry: ModuleRegistry,
    module_loaders: Vec<ModuleLoader>,
}

impl Drop for App {
    fn drop(&mut self) {
        self.registry.shutdown();
    }
}

impl App {
    pub fn with_module(mut self, path: ModuleSourcePath) -> Self {
        let cache_path = App::cache_path(&path);
        self.module_loaders
            .push(ModuleLoader::new(path, cache_path));

        self
    }

    pub fn run(&mut self) {}

    fn cache_path(source_path: &ModuleSourcePath) -> ModuleCachePath {
        ModuleCachePath(PathBuf::default())
    }
}
