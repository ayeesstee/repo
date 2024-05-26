// Copyright Ian Stewart 2024, All Rights Reserved.

use std::ffi::c_void;

use super::ModuleId;

/// The common module interface. Provides functions for managing the module's initialization and shutdown,
/// as well as a callback to get the module-specific API that is implemented by the loaded module.
#[repr(C)]
pub struct Module {
    /// The function used to get the ID of this module.
    ///
    /// # Returns
    /// The ID of this module.
    pub id_func: fn() -> ModuleId,

    /// The function used to initialize this module after it has been loaded into memory.
    ///
    /// # Arguments
    /// * `previous_state` - The previous state of this module before it was unloaded. Null if this is the first load.
    pub init_func: fn(previous_state: *const c_void),

    /// The function used to unload this module with the intention of reloading a newer version of it.
    ///
    /// # Returns
    /// The pointer to the serialized state of this module, to be passed to the new version that is later loaded.
    pub unload_func: fn() -> *const c_void,

    /// Shuts down this module, typically called during application exit.
    pub shutdown_func: fn(),

    /// Gets the pointer to the interface that this type of module implements.
    ///
    /// # Returns
    /// The pointer to the interface API for this module.
    pub interface_func: fn() -> *const c_void,
}
