// Copyright Ian Stewart 2024, All Rights Reserved.

use std::path::PathBuf;

mod loader;
pub use self::loader::ModuleLoader;

#[cfg(test)]
mod tests;

/// The full filesystem path to a module's shared object / DLL.
#[derive(Debug)]
pub struct ModuleSourcePath(pub PathBuf);

/// The full filesystem path to the module's cached shared object / DLL that is actually loaded into memory.
#[derive(Debug)]
pub struct ModuleCachePath(pub PathBuf);
