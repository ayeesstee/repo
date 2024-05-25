// Copyright Ian Stewart 2024, All Rights Reserved.

use crate::modules::{Module, ModuleId, ModuleRegistry};

#[derive(Debug)]
struct TestModule {
    id: ModuleId,
    some_payload: i32,
}

impl TestModule {
    pub fn new(id: ModuleId) -> Self {
        Self {
            id,
            some_payload: 0,
        }
    }
}

impl Module for TestModule {
    fn id(&self) -> ModuleId {
        self.id
    }
}

#[test]
fn single_registration_test() {
    let id = ModuleId("test");
    let module = Box::new(TestModule::new(id));
    let module_address = module.as_ref() as *const _;
    let mut registry = ModuleRegistry::default();

    // ModuleRegistry has the `Debug` trait, make code coverage happy.
    let _ = format!("{registry:#?}");

    registry.register(module);

    let retrieved_module = registry.get::<TestModule>(id);

    // Ensure we got a module instance back.
    assert!(retrieved_module.is_some());

    // Ensure the IDs match.
    assert!(retrieved_module.unwrap().id() == id);

    // Ensure that we're pointing to the address of the module we had registered.
    assert!(retrieved_module.unwrap() as *const _ == module_address);

    // Unregister the module and ensure we no longer get it back.
    registry.unregister(id);
    let retrieved_module = registry.get::<TestModule>(id);
    assert!(retrieved_module.is_none());
}

#[test]
#[should_panic]
fn duplicate_registration_test() {
    let id = ModuleId("test");
    let first_module = Box::new(TestModule::new(id));
    let second_module = Box::new(TestModule::new(id));

    let mut registry = ModuleRegistry::default();

    // Register the first module. This should succeed.
    registry.register(first_module);

    // This should panic, we already registered `first_module` with the same ID.
    registry.register(second_module);
}

#[test]
fn different_module_registration_test() {
    let first_id = ModuleId("Test1");
    let second_id = ModuleId("Test2");

    let first_module = Box::new(TestModule::new(first_id));
    let first_module_address = first_module.as_ref() as *const _;

    let second_module = Box::new(TestModule::new(second_id));
    let second_module_address = second_module.as_ref() as *const _;

    let mut registry = ModuleRegistry::default();

    registry.register(first_module);
    registry.register(second_module);

    let first_module = registry.get::<TestModule>(first_id).unwrap();
    assert!(first_module as *const _ == first_module_address);
    assert!(first_module.id() == first_id);

    let second_module = registry.get::<TestModule>(second_id).unwrap();
    assert!(second_module as *const _ == second_module_address);
    assert!(second_module.id() == second_id);
}

#[test]
#[should_panic]
fn bad_unregister_id_panics_test() {
    let bogus_id = ModuleId("hmm");
    let mut registry = ModuleRegistry::default();

    // This should panic, since no module has been registered with this ID.
    registry.unregister(bogus_id);
}

#[test]
#[should_panic]
fn duplicate_register_panics_test() {
    let id = ModuleId("id");
    let first_module = Box::new(TestModule::new(id));
    let second_module = Box::new(TestModule::new(id));

    let mut registry = ModuleRegistry::default();
    registry.register(first_module);

    // This should panic, as both modules have the same ID, and the first module wasn't unregistered first.
    registry.register(second_module);
}

#[test]
fn get_mut_test() {
    let id = ModuleId("id");
    let module = Box::new(TestModule::new(id));
    let mut registry = ModuleRegistry::default();

    registry.register(module);

    let mutable_module = registry.get_mut::<TestModule>(id).unwrap();
    mutable_module.some_payload = 42;

    assert!(registry.get_mut::<TestModule>(ModuleId("bogus")).is_none());
}
