// Copyright Ian Stewart 2024, All Rights Reserved.

mod module;
pub use self::module::Module;

mod registry;
pub use self::registry::ModuleRegistry;

#[cfg(test)]
mod tests;
