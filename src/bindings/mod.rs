#[cfg(not(target_arch = "wasm32"))] #[macro_use] mod native;
#[cfg(not(target_arch = "wasm32"))] pub use native::*;

#[cfg(target_arch = "wasm32")] #[macro_use] mod wasm32;
#[cfg(target_arch = "wasm32")] pub use wasm32::*;
