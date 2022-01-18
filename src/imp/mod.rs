#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm::{Stdout, Stderr};

#[cfg(target_arch = "mos")]
mod mos;
#[cfg(target_arch = "mos")]
pub use mos::{Stdout, Stderr};

#[cfg(not(any(all(target_arch = "wasm32", target_os = "unknown"), target_arch="mos")))]
mod std_c;
#[cfg(not(any(all(target_arch = "wasm32", target_os = "unknown"), target_arch="mos")))]
pub use std_c::{Stdout, Stderr};
