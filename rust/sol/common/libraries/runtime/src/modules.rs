// Copyright Ian Stewart 2024, All Rights Reserved.

use std::fmt::Display;

mod module;
pub use self::module::Module;

mod registry;
pub use self::registry::ModuleRegistry;

#[cfg(test)]
mod tests;

/// A unique identifier for a module. Used to lookup a desired module within a registry.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(i32);

impl Display for ModuleId {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
