//! readline impl using [noline](https://github.com/rustne-kretser/noline)

use core::arch::asm;

use super::Console;
use pac::UART0;

impl embedded_io::ErrorType for Console {
    type Error = core::convert::Infallible;
}

impl embedded_io::Read for Console {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        for b in buf.iter_mut() {
            while !UART0.lsr().read().dr() {
                unsafe { asm!("nop") };
            }

            *b = UART0.rbr().read().rbr();
        }

        Ok(buf.len())
    }
}

impl embedded_io::Write for Console {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for &c in buf {
            while !UART0.lsr().read().thre() {
                unsafe { asm!("nop") };
            }
            UART0.thr().write(|w| w.set_thr(c));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while !UART0.lsr().read().temt() {}

        Ok(())
    }
}
