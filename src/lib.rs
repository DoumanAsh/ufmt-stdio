//!C stdio writer for [ufmt](https://github.com/japaric/ufmt)

#![warn(missing_docs)]
#![no_std]

pub extern crate ufmt;

pub use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln};

#[cfg_attr(all(windows, target_env="msvc"), link(name="legacy_stdio_definitions", kind="dylib"))]
extern "C" {
    fn printf(fmt: *const i8, ...) -> i32;
}

#[derive(Copy, Clone)]
///Stdout wrapper
pub struct Stdout;

impl ufmt::uWrite for Stdout {
    type Error = i32;

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        //TODO: how the fuck to avoid allocating and append 0?
        let ret = unsafe {
            printf("%s\0".as_ptr() as *const i8, text.as_ptr() as *const i8)
        };

        if ret < 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }
}

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
