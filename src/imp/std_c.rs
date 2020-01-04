use crate::{Stdout, Stderr};

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

impl ufmt::uWrite for Stderr {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        let result = unsafe {
            libc::write(2, text.as_ptr() as *const _, text.len() as _)
        };

        if result < 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}
