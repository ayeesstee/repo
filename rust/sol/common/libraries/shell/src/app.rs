// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{
    env,
    path::{Path, PathBuf},
};

use sol_runtime::modules::ModuleRegistry;
use uuid::Uuid;

use crate::modules::{ModuleCachePath, ModuleLoader, ModuleSourcePath};

/// An application that provides a main loop and module registration.
#[derive(Debug, Default)]
pub struct App {
    /// The registered modules associated with this application.
    registry: ModuleRegistry,

    /// The backing module loaders for the modules associated with this application.
    module_loaders: Vec<ModuleLoader>,

    /// The unique identifier for this particular run of the application.
    uuid: Uuid,
}

impl Drop for App {
    fn drop(&mut self) {
        self.registry.shutdown();
    }
}

impl App {
    /// Adds a module to this application.
    ///
    /// # Arguments
    /// * `path` - The path to the module's binary.
    ///
    /// # Returns
    /// This application.
    pub fn with_module(mut self, path: ModuleSourcePath) -> Self {
        let cache_path = self.cache_path(&path);
        self.module_loaders
            .push(ModuleLoader::new(path, cache_path));

        self
    }

    /// Runs this application, executing its main loop.
    pub fn run(&mut self) {}

    /// Gets the cache path for a given module source path.
    ///
    /// # Arguments
    /// * `source_path` - The source path of the module.
    ///
    /// # Returns
    /// The cache path of the module.
    fn cache_path(&self, source_path: &ModuleSourcePath) -> ModuleCachePath {
        let tmp_dir = env::temp_dir();

        let mut uuid_part = PathBuf::new();
        uuid_part.push(self.uuid.to_string());

        let file_name = Path::file_name(&source_path.0).unwrap();

        let cache_path = tmp_dir.join(uuid_part).join(file_name);
        ModuleCachePath(cache_path)
    }
}
