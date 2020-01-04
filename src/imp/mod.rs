#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod std_c;
