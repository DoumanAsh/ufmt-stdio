//!Minimal printing facilities for [ufmt](https://github.com/japaric/ufmt)
//!
//!## Supported platforms:
//!
//!- `wasm` via `wasm-bindings`;
//!- `mos` microprocessors via linking `putchar`;
//!- `riscv32` via `esp-*` features (see details in `Features` section) or being no-op otherwise;
//!- All other platforms are built upon standard C library `write` function.
//!
//!## Features
//!
//!- `esp-uart` - Enables UART writer on `riscv32` targets. Mutually exclusive with `esp-jtag`. Requires user to provide symbols:
//!  - `ESP_UART_ADDR` (e.g. on ESP32-C3 it is `#[no_mangle] static ESP_UART_ADDR: usize = 0x40000068`).
//!- `esp-jtag` - Enables JTAG writer on `riscv32` targets. Mutually exclusive with `esp-jtag`. Requires user to provide symbols:
//!  - `SERIAL_JTAG_FIFO_REG` (e.g. on ESP32-C3 it is `#[no_mangle] static SERIAL_JTAG_FIFO_REG: usize = 0x60043000`)
//!  - `SERIAL_JTAG_CONF_REG` (e.g. on ESP32-C3 it is `#[no_mangle] static SERIAL_JTAG_CONF_REG: usize = 0x60043004`).

#![warn(missing_docs)]
#![no_std]

pub extern crate ufmt;

pub use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln};

#[cfg(windows)]
extern "system" {
    fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
}

///Initializes runtime, when necessary.
///
///On windows it calls `SetConsoleOutputCP(65001)` to enable unicode output
pub fn init() {
    #[cfg(windows)]
    {
        unsafe {
            SetConsoleOutputCP(65001);
        }
    }
}

mod imp;
pub use imp::{Stdout, Stderr};

///Prints to the stdout with newline
#[macro_export]
macro_rules! println {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout::new(), "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stdout::new(), $($arg)*);
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
///Prints to the stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout::new(), $($arg)*);
    };
}

///Prints to the stderr with newline
#[macro_export]
macro_rules! eprintln {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stderr::new(), "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stderr::new(), $($arg)*);
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
///Prints to the stderr
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stderr::new(), $($arg)*);
    };
}

///Prints to the stdout with newline in debug mode only
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! d_println {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout::new(), "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stdout::new(), $($arg)*);
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
///Prints to the stdout in debug mode only
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! d_print {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout::new(), $($arg)*);
    };
}

///Prints to the stdout with newline in debug mode only
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! d_println {
    ($($arg:tt)*) => {
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
///Prints to the stdout in debug mode only
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! d_print {
    ($($arg:tt)*) => {
    };
}
