use wasm_bindgen::prelude::wasm_bindgen;

use core::{ptr, cmp};
use core::mem::MaybeUninit;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

struct Buffer {
    buffer: MaybeUninit<[u8; 4096]>,
    len: usize,
}

impl Buffer {
    const fn new() -> Self {
        Self {
            buffer: MaybeUninit::uninit(),
            len: 0,
        }
    }

    #[inline]
    fn buffer(&self) -> &[u8; 4096] {
        unsafe {
            &*self.buffer.as_ptr()
        }
    }

    #[inline]
    fn buffer_mut(&mut self) -> &mut [u8; 4096] {
        unsafe {
            &mut *self.buffer.as_mut_ptr()
        }
    }

    fn write(&mut self, data: &[u8]) {
        let write_len = cmp::min(self.buffer().len(), data.len());
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.buffer_mut().as_mut_ptr().add(self.len), write_len)
        }
        self.len += write_len;
    }

    ///Takes output on newline, if last
    fn take(&mut self) -> &str {
        if self.len == 0 {
            return ""
        }

        let len = self.len;
        self.len = 0;
        let text = unsafe {
            core::str::from_utf8_unchecked(&self.buffer()[..len])
        };

        text
    }
}

///Stdout wrapper
pub struct Stdout {
    buffer: Buffer,
}

impl Stdout {
    ///Creates new instance;
    pub const fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }
}

///Stderr wrapper
pub struct Stderr {
    buffer: Buffer,
}

impl Stderr {
    ///Creates new instance;
    pub const fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }
}

impl ufmt::uWrite for Stdout {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= self.buffer.buffer().len());

        if self.buffer.len == self.buffer.buffer().len() || self.buffer.len + text.len() > self.buffer.buffer().len() {
            log(self.buffer.take());
        }

        self.buffer.write(text.as_bytes());

        if self.buffer.buffer()[self.buffer.len - 1] == b'\n' {
            self.buffer.len -= 1;
            log(self.buffer.take());
        }

        Ok(())
    }
}

impl ufmt::uWrite for Stderr {
    type Error = ();

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= self.buffer.buffer().len());

        if self.buffer.len == self.buffer.buffer().len() || self.buffer.len + text.len() > self.buffer.buffer().len() {
            error(self.buffer.take());
        }

        self.buffer.write(text.as_bytes());

        if self.buffer.buffer()[self.buffer.len - 1] == b'\n' {
            self.buffer.len -= 1;
            error(self.buffer.take());
        }

        Ok(())
    }
}
