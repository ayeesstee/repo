// Copyright Ian Stewart 2024, All Rights Reserved.
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
