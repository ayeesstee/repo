// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{collections::HashMap, ptr::null};

use crate::modules::{Module, ModuleId};

/// The central registry of all active modules.
#[derive(Debug, Default)]
pub struct ModuleRegistry {
    /// The lookup table of module identifiers to their respective loaded module.
    modules: HashMap<ModuleId, *const Module>,
}

impl ModuleRegistry {
    pub fn register(&mut self, id: ModuleId, module: *const Module) {
        if self.modules.contains_key(&id) {
            panic!("Module with id '{id}' is already registered");
        }

        self.modules.insert(id, module);
    }

    pub fn unregister(&mut self, id: ModuleId) {
        if !self.modules.contains_key(&id) {
            panic!("Attempting to unregister non-existent id '{id}'");
        }

        self.modules.remove(&id);
    }

    /// Gets the module interface for a given module ID, if it exists.
    ///
    /// # Arguments
    /// * `id` - The module ID to get the module interface for.
    ///
    /// # Returns
    /// The pointer to the module's API, if it exists.
    pub fn get<T>(&self, id: ModuleId) -> *const T {
        if let Some(n) = self.modules.get(&id) {
            let as_pointer = *n;
            unsafe {
                let interface_pointer = ((*as_pointer).interface_func)();
                interface_pointer as *const T
            }
        } else {
            null()
        }
    }

    /// Gets the low-level module pointer for a given module ID, if it exists.
    ///
    /// # Arguments
    /// * `id` - The module ID to get the module pointer for.
    ///
    /// # Returns
    /// The pointer to the low-level module, if it exists.
    pub fn get_raw(&self, id: ModuleId) -> *const Module {
        if let Some(n) = self.modules.get(&id) {
            n.clone()
        } else {
            null()
        }
    }

    pub fn shutdown(&mut self) {
        for module in self.modules.values() {
            let as_pointer = *module;
            if as_pointer != null() {
                unsafe {
                    ((*as_pointer).shutdown_func)();
                }
            }
        }

        self.modules.clear();
    }
}
