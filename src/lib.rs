//!C stdio writer for [ufmt](https://github.com/japaric/ufmt)

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

#[derive(Copy, Clone)]
///Stdout wrapper
pub struct Stdout;

#[derive(Copy, Clone)]
///Stderr wrapper
pub struct Stderr;

mod imp;

///Prints to the stdout with newline
#[macro_export]
macro_rules! println {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout, "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stdout, $($arg)*);
    };
}

///Prints to the stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout, $($arg)*);
    };
}

///Prints to the stderr with newline
#[macro_export]
macro_rules! eprintln {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stderr, "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stderr, $($arg)*);
    };
}

///Prints to the stderr
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stderr, $($arg)*);
    };
}

///Prints to the stdout with newline in debug mode only
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! d_println {
    () => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout, "\n");
    };
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwriteln!($crate::Stdout, $($arg)*);
    };
}

///Prints to the stdout in debug mode only
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! d_print {
    ($($arg:tt)*) => {
        let _ = $crate::ufmt::uwrite!($crate::Stdout, $($arg)*);
    };
}

///Prints to the stdout with newline in debug mode only
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! d_println {
    ($($arg:tt)*) => {
    };
}

///Prints to the stdout in debug mode only
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! d_print {
    ($($arg:tt)*) => {
    };
}
