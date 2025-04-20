use core::arch::asm;
use core::fmt;
use pac::UART0;

/// intended to be UART0
pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                while !UART0.lsr().read().thre() {
                    asm!("nop");
                }

                UART0.thr().write(|w| w.set_thr(*c));
            }
        }

        Ok(())
    }
}

impl Console {
    pub fn getchar(&self) -> u8 {
        unsafe {
            while UART0.lsr().read().dr() {
                asm!("nop");
            }

            UART0.rbr().read().rbr()
        }
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            while !UART0.lsr().read().thre() {
                asm!("nop");
            }

            UART0.thr().write(|w| w.set_thr(c));
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Console.write_fmt(args).unwrap();
}
