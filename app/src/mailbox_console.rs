use core::{arch::asm, ptr};

const MAILBOX_CONSOLE_BASE: usize = 0x010f_f000;
const STATUS_OFFSET: usize = 0x00;
const DATA_OFFSET: usize = 0x04;

const STATUS_EMPTY: u32 = 0;
const STATUS_FULL: u32 = 1;

#[inline(always)]
fn reg(offset: usize) -> *mut u32 {
    (MAILBOX_CONSOLE_BASE + offset) as *mut u32
}

pub fn sbi_console_putchar(byte: u8) {
    unsafe {
        while ptr::read_volatile(reg(STATUS_OFFSET)) != STATUS_EMPTY {
            asm!("nop");
        }

        ptr::write_volatile(reg(DATA_OFFSET), byte as u32);
        asm!("fence rw, rw");
        ptr::write_volatile(reg(STATUS_OFFSET), STATUS_FULL);
        asm!("fence rw, rw");
    }
}
