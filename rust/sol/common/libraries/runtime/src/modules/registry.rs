// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{collections::HashMap, ptr::null};

use crate::modules::{Module, ModuleId};

#[derive(Debug, Default)]
pub struct ModuleRegistry {
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

    pub fn get<T>(&self, id: ModuleId) -> Option<*const T> {
        if let Some(n) = self.modules.get(&id) {
            let as_pointer = *n;
            unsafe {
                let interface_pointer = ((*as_pointer).interface_func)();
                Some(interface_pointer as *const T)
            }
        } else {
            None
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
