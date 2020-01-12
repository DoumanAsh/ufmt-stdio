#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm::{Stdout, Stderr};

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod std_c;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use std_c::{Stdout, Stderr};
