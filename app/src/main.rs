#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::arch::asm;
use core::arch::global_asm;
use core::ptr;
use embedded_hal::delay::DelayNs;
use pac::UART3;

global_asm!(
    // no "c" here, the same as riscv-rt
    ".attribute arch, \"rv64im\"",
    ".section .start, \"ax\"
     .global _start

_start:
    //.word 0x00000000
    // init all registers
    li gp, 0
    // li sp, 0
    li tp, 0
    li t0, 0
    li t1, 0
    li t2, 0
    li s0, 0
    li s1, 0
    li a0, 0
    li a1, 0
    li a2, 0
    li a3, 0
    li a4, 0
    li a5, 0
    li a6, 0
    li a7, 0
    li s2, 0
    li s3, 0
    li s4, 0
    li s5, 0
    li s6, 0
    li s7, 0
    li s8, 0
    li s9, 0
    li s10, 0
    li s11, 0
    li t3, 0
    li t4, 0
    li t5, 0
    li t6, 0

    .option push
    .option norelax

    la gp, __global_pointer$
    .option pop

    la t1, __stack_start__
    addi sp, t1, -16

    // bss must be zeroed, ddr is dirty
    la  	t1, sbss
    la   	t2, ebss
1:  bgeu 	t1, t2, 1f
    sd   	zero, 0(t1)
    addi 	t1, t1, 8
    j    	1b
1:

    call _start_rust"
);

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
    () => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, "").unwrap();
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

#[derive(Debug)]
pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                while !UART3.lsr().read().thre() {
                    asm!("nop");
                }

                UART3.thr().write(|w| w.set_thr(*c));
            }
        }

        Ok(())
    }
}

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    let r = pac::UART3;

    for i in 0..100 {
        unsafe {
            ptr::write_volatile((0x2000_0000 + i * 4) as *mut u32, 0xAABBCCDD);
        }
    }

    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));
    r.thr().write(|w| w.set_thr(b'H' as u8));

    unsafe {
        ptr::write_volatile(0x911050c8 as *mut u32, 0x00000a8f); // UART3_TX. GPIO50
        ptr::write_volatile(0x911050cc as *mut u32, 0x80000bd0); // UART3_RX
    }

    let baud_rate = 115200;
    let clock_in = 50_000_000;
    let div = clock_in / (16 * baud_rate);

    // set baudrate
    r.lcr().write(|w| w.set_dlab(true));
    r.dlh().write(|w| w.set_dlh((div >> 8) as u8));
    r.dll().write(|w| w.set_dll(div as u8));
    r.lcr().write(|w| w.set_dlab(false));

    r.lcr().write(|w| {
        w.set_stop(pac::uart::vals::StopBits::STOP1);
        w.set_wls(pac::uart::vals::DataBits::BIT8);
        w.set_pen(false);
    });
    r.fcr().write(|w| w.set_fifoe(true));
    // no modem
    r.mcr().write(|w| {
        w.set_out1(false);
        w.set_out2(false);
    });
    // no interrupt
    r.ier().modify(|w| w.0 = 0);

    let mut delay = riscv::delay::McycleDelay::new(1_600_000_000);

    println!("\r\n\r\nRust 2nd stage on CPU1");

    cpuid();

    loop {
        println!(
            "!! Hello, world! {} {:016x}",
            riscv::register::mhartid::read(),
            riscv::register::mcycle::read64()
        );

        delay.delay_ms(1000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {:?}", _info);
    loop {}
}

pub fn cpuid() {
    let mstatus = riscv::register::mstatus::read();
    println!("mstatus: {:016x}", mstatus.bits());

    let mie = riscv::register::mie::read();
    println!("mie: {:016x}", mie.bits());

    let mip = riscv::register::mip::read();
    println!("mip: {:016x}", mip.bits());

    let misa = riscv::register::misa::read().unwrap();

    println!("misa: {:x}", misa.bits());
    print!("  RV64");
    for c in 'A'..='Z' {
        if misa.has_extension(c) {
            print!("{}", c);
        }
    }
    println!();

    let mvendorid = riscv::register::mvendorid::read().unwrap();
    println!("mvendorid: {:x}", mvendorid.bits());

    let marchid = riscv::register::marchid::read().unwrap();
    println!("marchid: {:x}", marchid.bits());

    let mhartid = riscv::register::mhartid::read();
    println!("mhartid: {:x}", mhartid);

    let mut cpuid0: u64;
    let mut cpuid1: u64;
    let mut cpuid2: u64;
    unsafe {
        asm!("
        csrr {0}, 0xfc0
        csrr {1}, 0xfc0
        csrr {2}, 0xfc0
    ", out(reg) cpuid0, out(reg) cpuid1, out(reg) cpuid2);
    }
    println!("cpuid: {:08x} {:08x} {:08x}", cpuid0, cpuid1, cpuid2);
}
