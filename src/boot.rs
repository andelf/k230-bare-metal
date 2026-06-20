use core::{
    arch::asm,
    ptr::{self, slice_from_raw_parts},
};

use embedded_io::ReadReady;

use core::fmt::Write;

use crate::{getchar, putc, serial::Uart, Console, CPU0_CORE_CLK};

pub const SFL_MAGIC_REQ: &[u8] = b"sL5DdSMmkekro\n";
pub const SFL_MAGIC_ACK: &[u8] = b"z6IHG7cYDID6o\n";

#[derive(Debug, PartialEq)]
pub enum Ack {
    Timeout,
    Cancelled,
    Ok,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct SflFrame {
    payload_length: u8,
    crc: [u8; 2],
    cmd: u8,
    payload: [u8; 255],
}

// General commands
pub const SFL_CMD_ABORT: u8 = 0x00;
pub const SFL_CMD_LOAD: u8 = 0x01;
pub const SFL_CMD_JUMP: u8 = 0x02;

// Replies
pub const SFL_ACK_SUCCESS: u8 = b'K';
pub const SFL_ACK_CRCERROR: u8 = b'C';
pub const SFL_ACK_UNKNOWN: u8 = b'U';
pub const SFL_ACK_ERROR: u8 = b'E';

const CPU0_FIRMWARE_ADDR: usize = 0x8030_0000;
const CPU0_FIRMWARE_MAX_LEN: usize = 0x0008_0000;
const CPU0_UPDATE_STAGING_ADDR: usize = 0x8020_0000;
const CPU0_UPDATE_TRAMPOLINE_ADDR: usize = 0x803f_0000;

const CPU0_UPDATE_TRAMPOLINE: [u8; 28] = [
    0x09, 0xca, 0x83, 0x42, 0x05, 0x00, 0x23, 0x80, 0x55, 0x00, 0x05, 0x05, 0x85, 0x05, 0x7d, 0x16,
    0xc5, 0xbf, 0x0f, 0x00, 0x30, 0x03, 0x0f, 0x10, 0x00, 0x00, 0x82, 0x86,
];

#[derive(Copy, Clone, Eq, PartialEq)]
enum SerialMode {
    BigCore,
    Cpu0Update,
}

pub fn jump_big_core(jump_addr: u32) -> ! {
    crate::mailbox_console::init();

    unsafe {
        ptr::write_volatile(0x91102104 as *mut u32, jump_addr);
        ptr::write_volatile(0x9110100c as *mut u32, 0x10001000);
        ptr::write_volatile(0x9110100c as *mut u32, 0x10001);
        ptr::write_volatile(0x9110100c as *mut u32, 0x10000);
    }

    loop {
        if let Some(byte) = crate::mailbox_console::try_read_byte() {
            putc(byte);
        } else {
            unsafe {
                asm!("nop");
            }
        }
    }
}

pub struct DebugConsole;

unsafe impl crate::serial::DevId for DebugConsole {
    const ADDRESS: *const () = 0x9140_3000 as *const (); // UART3
}

// simulate LiteX's serialboot
pub fn litex_term_serial_boot() -> i32 {
    serial_loader(SerialMode::BigCore)
}

pub fn serial_firmware_update() -> i32 {
    serial_loader(SerialMode::Cpu0Update)
}

fn serial_loader(mode: SerialMode) -> i32 {
    match mode {
        SerialMode::BigCore => println!("Booting from serial..."),
        SerialMode::Cpu0Update => {
            println!("Updating CPU0 firmware from serial...");
            println!(
                "Upload firmware.bin to 0x{:08x}, max {} bytes.",
                CPU0_UPDATE_STAGING_ADDR, CPU0_FIRMWARE_MAX_LEN
            );
        }
    }
    println!("Press Q or ESC to abort boot completely.");

    let mut debug_shell: Uart<'_, DebugConsole> = Uart::new();
    // already initialized in main.rs

    for &c in SFL_MAGIC_REQ {
        putc(c);
    }

    match check_ack() {
        Ack::Ok => (),
        Ack::Cancelled => {
            println!("Cancelled");
            return 0;
        }
        Ack::Timeout => {
            println!("Timeout");
            return 1;
        }
    }

    let mut last_activity = riscv::register::mcycle::read64();
    let max_idle = (CPU0_CORE_CLK * 3) as u64;

    let mut frame = SflFrame {
        payload_length: 0,
        crc: [0; 2],
        cmd: 0,
        payload: [0; 255],
    };
    let mut update_loaded_end = CPU0_UPDATE_STAGING_ADDR;

    // Ack
    let mut failures = 0;
    loop {
        // Get one Frame
        let mut i = 0;
        let mut timeout = true;

        while (riscv::register::mcycle::read64() - last_activity) < max_idle {
            if Console.read_ready().unwrap() {
                if i == 0 {
                    frame.payload_length = getchar();
                    last_activity = riscv::register::mcycle::read64();
                } else if i == 1 {
                    frame.crc[0] = getchar();
                } else if i == 2 {
                    frame.crc[1] = getchar();
                } else if i == 3 {
                    frame.cmd = getchar();
                } else {
                    // >= 4
                    frame.payload[i - 4] = getchar();
                    if i as u8 == (frame.payload_length + 4 - 1) {
                        timeout = false;
                        break;
                    }
                }
                i += 1;
            }
        }

        // check timeout
        if timeout {
            putc(SFL_ACK_ERROR);
            continue;
        }

        // check frame crc
        let received_crc = ((frame.crc[0] as u16) << 8) | (frame.crc[1] as u16);
        let computed_crc = unsafe {
            crc16(&*slice_from_raw_parts(
                &frame.cmd,
                (frame.payload_length + 1) as usize,
            ))
        };

        //writeln!(debug_shell, "Received CRC: 0x{:04x}", received_crc).unwrap();
        //writeln!(debug_shell, "Computed CRC: 0x{:04x}", computed_crc).unwrap();
        // writeln!(debug_shell, "{:02x?}", frame).unwrap();

        if computed_crc != received_crc {
            putc(SFL_ACK_CRCERROR);

            failures += 1;
            if failures >= 256 {
                println!("Too many errors, aborting");
                return 1;
            }
            continue;
        }

        // Execute frame cmd
        match frame.cmd {
            SFL_CMD_ABORT => {
                // reset fails
                putc(SFL_ACK_SUCCESS);
                return 1;
            }
            SFL_CMD_LOAD => {
                failures = 0;

                let load_addr = u32::from_be_bytes(frame.payload[0..4].try_into().unwrap());
                let copy_len = frame.payload_length as usize - 4;

                if mode == SerialMode::Cpu0Update
                    && !cpu0_update_range_valid(load_addr as usize, copy_len)
                {
                    putc(SFL_ACK_ERROR);
                    continue;
                }

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        frame.payload[4..].as_ptr(),
                        load_addr as *mut u8,
                        copy_len,
                    );
                }

                if mode == SerialMode::Cpu0Update {
                    update_loaded_end = update_loaded_end.max(load_addr as usize + copy_len);
                }

                putc(SFL_ACK_SUCCESS);
            }
            SFL_CMD_JUMP => {
                let jump_addr = u32::from_be_bytes(frame.payload[0..4].try_into().unwrap());

                putc(SFL_ACK_SUCCESS);

                match mode {
                    SerialMode::BigCore => {
                        writeln!(debug_shell, "\r\nJumping to 0x{:08x}...", jump_addr).unwrap();
                        println!("Jumping to 0x{:08x}...", jump_addr);
                        jump_big_core(jump_addr);
                    }
                    SerialMode::Cpu0Update => {
                        if jump_addr as usize != CPU0_UPDATE_STAGING_ADDR
                            || update_loaded_end <= CPU0_UPDATE_STAGING_ADDR
                        {
                            println!("Invalid CPU0 update image");
                            return 1;
                        }

                        let len = update_loaded_end - CPU0_UPDATE_STAGING_ADDR;
                        println!(
                            "Applying CPU0 firmware update: {} bytes 0x{:08x} -> 0x{:08x}",
                            len, CPU0_UPDATE_STAGING_ADDR, CPU0_FIRMWARE_ADDR
                        );
                        apply_cpu0_update(len);
                    }
                }
            }
            _ => {
                failures += 1;

                putc(SFL_ACK_UNKNOWN);

                if failures >= 256 {
                    println!("Too many errors, aborting");
                    return 1;
                }
            }
        }
    } // outer loop
}

fn cpu0_update_range_valid(load_addr: usize, len: usize) -> bool {
    if len == 0 {
        return true;
    }

    let Some(end) = load_addr.checked_add(len) else {
        return false;
    };

    load_addr >= CPU0_UPDATE_STAGING_ADDR && end <= CPU0_UPDATE_STAGING_ADDR + CPU0_FIRMWARE_MAX_LEN
}

fn apply_cpu0_update(len: usize) -> ! {
    if len == 0 || len > CPU0_FIRMWARE_MAX_LEN {
        println!("Invalid CPU0 update length: {}", len);
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    unsafe {
        core::ptr::copy_nonoverlapping(
            CPU0_UPDATE_TRAMPOLINE.as_ptr(),
            CPU0_UPDATE_TRAMPOLINE_ADDR as *mut u8,
            CPU0_UPDATE_TRAMPOLINE.len(),
        );

        asm!("fence rw, rw", "fence.i");

        let trampoline: extern "C" fn(usize, usize, usize, usize) -> ! =
            core::mem::transmute(CPU0_UPDATE_TRAMPOLINE_ADDR);

        trampoline(
            CPU0_UPDATE_STAGING_ADDR,
            CPU0_FIRMWARE_ADDR,
            len,
            CPU0_FIRMWARE_ADDR,
        );
    }
}

fn check_ack() -> Ack {
    let started = riscv::register::mcycle::read64();
    let deadline = started + (CPU0_CORE_CLK as u64) * 3;

    let mut uart = Console;

    let mut recongnized = 0;

    while riscv::register::mcycle::read64() <= deadline {
        if uart.read_ready().unwrap() {
            let c = getchar();
            if c == b'Q' || c == b'\x1b' {
                return Ack::Cancelled;
            }

            if c == SFL_MAGIC_ACK[recongnized] {
                recongnized += 1;

                if recongnized == SFL_MAGIC_ACK.len() {
                    return Ack::Ok;
                }
            } else {
                if c == SFL_MAGIC_ACK[0] {
                    recongnized = 1;
                } else {
                    recongnized = 0;
                }
            }
        }
    }

    Ack::Timeout
}

fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;

    for &byte in data {
        let mut x = ((crc >> 8) as u8) ^ byte;
        x ^= x >> 4;
        crc = (crc << 8) ^ ((x as u16) << 12) ^ ((x as u16) << 5) ^ (x as u16);
    }

    crc
}
