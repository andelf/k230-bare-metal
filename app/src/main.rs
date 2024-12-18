#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::arch::asm;
use core::arch::global_asm;
use core::ptr;
use embedded_hal::delay::DelayNs;
use pac::UART3;

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

pub mod ruacpu;
pub mod rvv;
pub mod rt;


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

fn init_uart() {
    let r = UART3;

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
}

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    init_uart();

    let mut delay = riscv::delay::McycleDelay::new(1_600_000_000);

    println!("\r\n\r\n2nd stage on CPU1");

    cpuid();

    // read csr 0xc22
    let mut vlenb = 0;
    asm!("csrr {0}, 0xc22", out(reg) vlenb);

    println!("vlenb: {:08x}, {}", vlenb, vlenb * 8);

    println!("-= RVV Demo 1 =-");
    rvv::rvv_demo1();

    println!("-= RVV Demo 2 =-");
    rvv::rvv_demo2();

    println!("-= RVV Demo 3 =-");
    rvv::rvv_demo3();

    println!("Rua CPU:");

    // asm!("dcache.cpa a0");

    ruacpu::detect();

    loop {
        println!(
            "!! Hello, world! {} {:016x}",
            riscv::register::mhartid::read(),
            riscv::register::mcycle::read64()
        );

        delay.delay_ms(2000);
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

    let mut mxstatus: u64;
    unsafe {
        asm!("csrr {0}, 0x7C0", out(reg) mxstatus);
    }
    println!("mxstatus: {:08x}", mxstatus);
}
