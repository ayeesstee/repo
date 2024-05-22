// Copyright Ian Stewart 2024, All Rights Reserved.

use mimalloc::MiMalloc;

/// The implementation to use for the global allocator.
pub type Global = MiMalloc;
