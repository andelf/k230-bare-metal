use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub use k230_boot::console::Console;
lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console);
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

fn without_interrupts<T>(f: impl Fn() -> T) -> T {
    riscv::interrupt::disable();
    let t = f();
    unsafe { riscv::interrupt::enable() };
    return t;
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    // To prevent dead lock when `print` is called inside an interrupt handler.
    without_interrupts(|| {
        CONSOLE.lock().write_fmt(args).unwrap();
    });
}

pub fn getchar() -> u8 {
    without_interrupts(|| CONSOLE.lock().getchar())
}

pub fn putc(c: u8) {
    without_interrupts(|| {
        CONSOLE.lock().putc(c);
    })
}
