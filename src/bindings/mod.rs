// #[cfg(target_arch = "wasm32")]
// pub use crate::wasm32::utils;

// #[cfg(not(target_arch = "wasm32"))]

#[macro_use]
mod wasm32;

pub use wasm32::*;
