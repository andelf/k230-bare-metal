use core::{arch::asm, ptr};

const MAILBOX_CONSOLE_BASE: usize = 0x010f_f000;
const STATUS_OFFSET: usize = 0x00;
const DATA_OFFSET: usize = 0x04;
const MAGIC_OFFSET: usize = 0x08;

const STATUS_EMPTY: u32 = 0;
const STATUS_FULL: u32 = 1;
const MAGIC: u32 = 0x4d42_5831;

#[inline(always)]
fn reg(offset: usize) -> *mut u32 {
    (MAILBOX_CONSOLE_BASE + offset) as *mut u32
}

pub fn init() {
    unsafe {
        ptr::write_volatile(reg(DATA_OFFSET), 0);
        ptr::write_volatile(reg(STATUS_OFFSET), STATUS_EMPTY);
        ptr::write_volatile(reg(MAGIC_OFFSET), MAGIC);
        asm!("fence rw, rw");
    }
}

pub fn try_read_byte() -> Option<u8> {
    unsafe {
        if ptr::read_volatile(reg(STATUS_OFFSET)) != STATUS_FULL {
            return None;
        }

        asm!("fence rw, rw");
        let byte = ptr::read_volatile(reg(DATA_OFFSET)) as u8;
        ptr::write_volatile(reg(STATUS_OFFSET), STATUS_EMPTY);
        asm!("fence rw, rw");

        Some(byte)
    }
}
