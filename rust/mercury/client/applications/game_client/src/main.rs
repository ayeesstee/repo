// Copyright Ian Stewart 2024, All Rights Reserved.

use sol_runtime::memory::Global;

#[global_allocator]
static GLOBAL: Global = Global {};

fn main() {}
