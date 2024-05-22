// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{
    fs,
    path::{Path, PathBuf},
};

use libloading::Library;
use sol_runtime::modules::ModuleRegistry;

/// A module loader that encapsulates caching a given module DLL/Shared Object, and the
/// loading/unloading of the module trait from it.
#[derive(Debug)]
pub struct ModuleLoader {
    /// The original file path to the module, before it is cached off.
    /// We cache the module elsewhere so that the instance at this path can be rewritten
    /// at runtime, allowing hot code reload.
    source_path: PathBuf,

    /// The file path the module will be cached at and then loaded into memory.
    cache_path: PathBuf,

    /// The currently loaded instance of the module library.
    library: Option<Library>,
}

impl ModuleLoader {
    /// Creates a module using the given source path and cache path for the library.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The file path to the module's DLL/shared object.
    /// * `cache_path` - The file path the module will be copied to, to prevent locking the original.
    #[inline]
    pub fn new(source_path: &Path, cache_path: &Path) -> Self {
        Self {
            source_path: source_path.to_owned(),
            cache_path: cache_path.to_owned(),
            library: None,
        }
    }

    /// Reloads the module associated with this loader.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry the module should be unloaded/loaded against.
    #[inline]
    pub fn reload(&mut self, registry: &mut ModuleRegistry) {
        self.unload(registry);
        self.load(registry);
    }

    /// Unloads the module associated with this loader.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry the module should be unregistered from.
    fn unload(&mut self, registry: &mut ModuleRegistry) {
        // Unregister from the module registry.
        if let Some(old_instance) = &self.module {
            registry.unregister(old_instance.id());
        }

        // Drop our current library instance.
        self.library = None;

        // Delete the cached module file, if it exists.
        if let Some(cache_path) = &self.cache_path {
            if cache_path.exists() {
                fs::remove_file(cache_path)
                    .expect("Failed deleting module at path '{self.cache_path}'!");
            }
        }
    }

    fn load(&mut self, registry: &mut ModuleRegistry) {
        // Copy the module file from the source path to the cached path.
        let library_path = if let Some(cache_path) = &self.cache_path {
            fs::copy(&self.source_path, cache_path)
                .expect("Failed to copy module from '{self.source_path}' to '{self.cache_path}'!");
            cache_path
        } else {
            &self.source_path
        };

        unsafe {
            self.library = Some(
                Library::new(library_path)
                    .expect("Failed to load module at path '{self.cache_path}'!"),
            );
        }
    }
}
