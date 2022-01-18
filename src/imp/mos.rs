///Stdout wrapper
pub struct Stdout {
}

impl Stdout {
    ///Creates new instance;
    pub const fn new() -> Self {
        Self {
        }
    }
}

///Stderr wrapper
pub struct Stderr {
}

impl Stderr {
    ///Creates new instance;
    pub const fn new() -> Self {
        Self {
        }
    }
}

extern "C" {
    /// provided by https://github.com/llvm-mos/llvm-mos-sdk
    fn putchar(c: u8);
}

#[inline(always)]
fn write(text: &str) {
    text.bytes().for_each(|b| unsafe { putchar(b) });
}

impl ufmt::uWrite for Stdout {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        write(text);
        Ok(())
    }
}

impl ufmt::uWrite for Stderr {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        write(text);
        Ok(())
    }
}
