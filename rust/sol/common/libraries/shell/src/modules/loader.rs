// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{
    ffi::c_void,
    fs::{self, create_dir},
    ptr::null,
};

use libloading::{Library, Symbol};
use sol_runtime::modules::{Module, ModuleId, ModuleRegistry};

use super::{ModuleCachePath, ModuleSourcePath};

/// A module loader that encapsulates caching a given module DLL/Shared Object, and the
/// loading/unloading of the module trait from it.
#[derive(Debug)]
pub struct ModuleLoader {
    /// The original file path to the module, before it is cached off.
    /// We cache the module elsewhere so that the instance at this path can be rewritten
    /// at runtime, allowing hot code reload.
    source_path: ModuleSourcePath,

    /// The file path the module will be cached at and then loaded into memory.
    cache_path: ModuleCachePath,

    /// The currently loaded instance of the module library.
    library: Option<Library>,

    /// The ID of the loaded module.
    id: Option<ModuleId>,
}

impl ModuleLoader {
    /// Creates a module loader using the given source path and cache path for the library.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The file path to the module's DLL/shared object.
    /// * `cache_path` - The file path the module will be copied to, to prevent locking the original.
    ///
    /// # Returns
    /// The new module loader.
    pub fn new(source_path: ModuleSourcePath, cache_path: ModuleCachePath) -> Self {
        // Ensure our module file exists.
        if !source_path.0.is_file() {
            let as_str = source_path.0.as_os_str().to_str().unwrap();
            panic!("Provided source path '{as_str}' is not a file");
        }

        // Ensure the directory for the cached copy of this module exists.
        let cache_dir = cache_path.0.parent().unwrap();
        if !cache_dir.exists() {
            create_dir(cache_dir).expect("Failed to create cache directory '{cache_dir}'");
        }

        Self {
            source_path,
            cache_path,
            library: None,
            id: None,
        }
    }

    /// Updates this loader, reloading the module, if necessary.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry to unload and reload the module against.
    pub fn update(&mut self, registry: &mut ModuleRegistry) {
        self.reload(registry);
    }

    /// Reloads the module associated with this loader.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry the module should be unloaded/loaded against.
    #[inline]
    pub fn reload(&mut self, registry: &mut ModuleRegistry) {
        let serialized_state = self.unload(registry);
        self.load(registry, serialized_state);
    }

    /// Unloads the module associated with this loader.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry the module should be unregistered from.
    fn unload(&mut self, registry: &mut ModuleRegistry) -> *const c_void {
        // If we haven't yet loaded a library, then there is no work to be done.
        if self.library.is_none() {
            return null();
        }

        // Invariant: If we have a loaded library, we should have a valid module ID.
        //            We check self.library above, so this unwrap *should* be safe.
        let id = self.id.unwrap();

        // Get the existing module for our module ID from the registry.
        let module = registry.get_raw(id);

        // If we retrieved a valid module back, unload it and capture the resulting serialized state.
        let serialized_state = if module != null() {
            unsafe { ((*module).unload_func)() }
        } else {
            null()
        };

        // We've unloaded the module, unregister it from the registry.
        registry.unregister(id);

        // Drop our current library instance.
        self.library = None;

        // Delete the cached module file.
        fs::remove_file(&self.cache_path.0)
            .expect("Failed deleting cached module at path {self.cache_path}");

        serialized_state
    }

    /// Loads the module associated with this loader.
    ///
    /// # Arguments
    ///
    /// * `registry` - The module registry the loaded module should be registered with.
    /// * `previous_state` - The serialized state of the previous version of this module.
    fn load(&mut self, registry: &mut ModuleRegistry, previous_state: *const c_void) {
        // Copy the the module located at the source path to the cache path.
        fs::copy(&self.source_path.0, &self.cache_path.0)
            .expect("Failed copying module from {self.source_path} to {self.cache_path}");

        // Load the cached module into memory.
        unsafe {
            self.library = Some(
                Library::new(&self.cache_path.0)
                    .expect("Failed to load module at path '{self.cache_path}'"),
            );
        }

        // Initialize and register the new module.
        unsafe {
            let get_module_func: Symbol<extern "C" fn() -> *const Module> = self
                .library
                .as_ref()
                .unwrap()
                .get(b"module_api")
                .expect("Failed to retrieve module API function from {self.cache_path}");

            let module = (get_module_func)();
            if module == null() {
                panic!(
                    "Failed to retrieve module API from '{0}'",
                    &self.cache_path.0.as_os_str().to_str().unwrap()
                );
            } else {
                ((*module).init_func)(previous_state);

                self.id = Some(((*module).id_func)());
                registry.register(self.id.unwrap(), module);
            }
        }
    }
}
