// Copyright Ian Stewart 2024, All Rights Reserved.

/// Helper macro to generate the API surface for getting and setting a given module.
/// Reduces the amount of boilerplate code required when adding a new module interface
/// to the registry.
///
/// # Arguments
///
/// * `name` - The name of the backing variable in the registry. This name is then used
///            in naming the generated getters and setters, e.g.
///            'foo(), foo_mut(), set_foo(...)'.
/// * 'trait_type' - The trait to be used for this variable's API.
macro_rules! module_api {
    ($variable:ident, $trait_type:ty) => {
        /// Gets an immutable reference, if a module of this type has been registered.
        #[inline]
        pub fn $variable(&self) -> Option<&$trait_type> {
            if let Some(instance) = &self.$variable {
                Some(instance.as_ref())
            } else {
                None
            }
        }

        /// Gets a mutable reference, if a module of this type has been registered.
        #[inline]
        pub fn $variable_mut(&mut self) -> Option<&mut dyn $trait_type> {
            if let Some(instance) = &mut self.<$variable> {
                Some(instance.as_mut())
            } else {
                None
            }
        }

        /// Sets the instance to be used for this module interface.
        #[inline]
        pub fn set_$variable(&mut self, val: Option<Box<dyn $trait_type>>) {
            self.$variable = val;
        }
    };
}

/// The macro for generating the C API boilerplate necessary to expose an implementation of
/// a module trait to processes loading the module's library dynamically at runtime.
#[macro_export]
macro_rules! module_impl {
    ($module_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_module(
            registry: *mut std::ffi::c_void,
            previous_state: *const std::ffi::c_void,
        ) {
        }

        pub extern "C" fn destroy_module(
            instance: *mut std::ffi::c_void,
        ) -> *const std::ffi::c_void {
            std::ptr::null()
        }

        /*#[no_mangle]
        pub extern "C" fn create_module() -> *mut std::ffi::c_void {
            // Use the Box API to heap allocate the module type.
            // Double allocate, since we're allocating the module as a trait type.
            // This is necessary, since a trait pointer is a fat pointer, but our
            // C API is only returning a regular pointer. So we box the fat pointer,
            // which provides us with a regular pointer that can then be casted by the caller.
            let double_box: Box<Box<dyn std::any::Any>> =
                Box::new(Box::new(<$module_type>::default()));

            // Leak our boxes so they don't get deallocated with this function returns.
            let single_box = Box::leak(double_box);

            // Decay down to void pointer.
            let mut_ref = single_box.as_mut();
            let ptr: *mut dyn std::any::Any = mut_ref;

            ptr as *mut std::ffi::c_void
        }

        #[no_mangle]
        pub extern "C" fn destroy_module(instance: *mut std::ffi::c_void) {
            unsafe {
                let as_box: Box<Box<dyn std::any::Any>> =
                    Box::from_raw(instance as *mut Box<dyn std::any::Any>);
            }
            // Box drops here, deallocating the module.
        }*/
    };
}
