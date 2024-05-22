// Copyright Ian Stewart 2024, All Rights Reserved.

use std::{any::Any, fmt::Debug};

pub trait Module: Debug + Any {
    fn init(&mut self, _serialized_state: Option<Box<dyn Any>>);

    fn unload(&mut self) -> Option<Box<dyn Any>>;

    fn shutdown(&mut self);
}
