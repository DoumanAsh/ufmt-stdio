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

impl ufmt::uWrite for Stdout {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        let result = unsafe {
            libc::write(1, text.as_ptr() as *const _, text.len() as _)
        };

        if result < 0 {
            Err(())
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
