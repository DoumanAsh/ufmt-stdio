#[cfg(feature = "esp-uart")]
mod uart {
    extern "C" {
        static ESP_UART_ADDR: usize;
    }

    #[inline(always)]
    pub fn write(text: &str) {
        critical_section::with(|_| {
            for byte in text.as_bytes() {
                unsafe {
                    let uart_tx_one_char: unsafe extern "C" fn(u8) -> i32 = core::mem::transmute(ESP_UART_ADDR);
                    uart_tx_one_char(*byte)
                };
            }
        })
    }
}

#[cfg(feature = "esp-jtag")]
mod jtag {
    const TIMEOUT_ITERATIONS: usize = 5_000;
    extern "C" {
        static SERIAL_JTAG_FIFO_REG: usize;
        static SERIAL_JTAG_CONF_REG: usize;
    }

    //Referenced from https://github.com/esp-rs/esp-println/blob/main/src/lib.rs
    pub fn write(text: &str) {
        critical_section::with(|_| {
            let (fifo, conf) = unsafe {
                (SERIAL_JTAG_FIFO_REG as *mut u32, SERIAL_JTAG_CONF_REG as *mut u32)
            };

            if unsafe { conf.read_volatile() } & 0b011 == 0b000 {
                // still wasn't able to drain the FIFO - early return
                return;
            }

            // todo 64 byte chunks max
            for chunk in text.as_bytes().chunks(32) {
                unsafe {
                    for &byte in chunk {
                        fifo.write_volatile(byte as u32);
                    }
                    conf.write_volatile(0b001);

                    let mut timeout = TIMEOUT_ITERATIONS;
                    while conf.read_volatile() & 0b011 == 0b000 {
                        // wait
                        timeout -= 1;
                        if timeout == 0 {
                            return;
                        }
                    }
                }
            }
        }) //critical_section
    }
}

#[cfg(feature = "esp-uart")]
pub use uart::write;

#[cfg(feature = "esp-jtag")]
pub use jtag::write;

#[cfg(not(any(feature = "esp-uart", feature = "esp-jtag")))]
#[inline(always)]
fn write(_: &str) {
}

///Stdout wrapper
pub struct Stdout {
}

impl Stdout {
    #[inline(always)]
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
    #[inline(always)]
    ///Creates new instance;
    pub const fn new() -> Self {
        Self {
        }
    }
}

impl ufmt::uWrite for Stdout {
    type Error = ();

    #[inline(always)]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        write(text);
        Ok(())
    }
}

impl ufmt::uWrite for Stderr {
    type Error = ();

    #[inline(always)]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        write(text);
        Ok(())
    }
}
