#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::fmt::{self, Write as _};
use core::{
    arch::{asm, global_asm},
    ptr,
};
use embedded_hal::delay::DelayNs;
use pac::UART0;

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

pub mod ddr_init;

// 2-7 clock frequency
pub const OSC24M: u32 = 24_000_000;
pub const PLL0_OCLK: u32 = 1_600_000_000;
pub const PLL1_OCLK: u32 = 2_376_000_000;
pub const PLL2_OCLK: u32 = 2_666_666_666;
pub const PLL3_OCLK: u32 = 1_600_000_000;

/// cpu0_core_clk
pub const CPU0_CORE_CLK: u32 = 800_000_000;
pub const CPU0_PLICLK: u32 = 400_000_000;
pub const CPU0_ACLK: u32 = 400_000_000;
pub const CPU0_PCLK: u32 = 200_000_000;

pub const UART0_SCLK: u32 = 50_000_000;
pub const UART1_SCLK: u32 = 50_000_000;
pub const UART2_SCLK: u32 = 50_000_000;
pub const UART3_SCLK: u32 = 50_000_000;
pub const UART4_SCLK: u32 = 50_000_000;

pub const DDRC_CPRE_CLK: u32 = 800_000_000;

// for MTIME
pub const STC_CLK: u32 = 27_000_000;

/// Parse cfg attributes inside a global_asm call.
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

cfg_global_asm!(
    // no "c" here, the same as riscv-rt
    ".attribute arch, \"rv64im\"",
    ".section .start, \"ax\"
     .global _start

_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    la t1, __stack_start__
    addi sp, t1, -16

    la t0, _start_trap_rust
    csrw mtvec, t0
    csrw mie, zero

    csrr t0, mhartid
    bne t0, zero, hart1

    li    t0, 0x00001800
    csrw  mstatus, t0

    call _early_init

    call _start_rust

1:
    j 1b

hart1:
    wfi
    j hart1
    ",
);

// weak functions
cfg_global_asm!(
    ".weak __pre_init
__pre_init:
    ret",
    ".weak _mp_hook
_mp_hook:
    beqz a0, 2f // if hartid is 0, return true
1:  wfi // Otherwise, wait for interrupt in a loop
    j 1b
2:  li a0, 1
    ret",
);

#[link_section = ".trap"]
#[no_mangle]
unsafe extern "riscv-interrupt-m" fn _start_trap_rust() {
    // riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(1000);

    println!("trap!");

    let mcause = riscv::register::mcause::read();
    println!("mstatus: {:016x}", riscv::register::mstatus::read().bits());
    println!("mcause:  {:016x}", riscv::register::mcause::read().bits());
    println!("mtval:   {:016x}", riscv::register::mtval::read());
    println!("mepc:    {:016x}", riscv::register::mepc::read());

    if mcause.is_interrupt() && mcause.code() == riscv::interrupt::Interrupt::MachineSoft as _ {
        println!("Machine Software Interrupt");

        pac::CLINT.msip(0).write(|w| w.set_msip(false));

        return;
    } else if mcause.is_interrupt()
        && mcause.code() == riscv::interrupt::Interrupt::MachineTimer as _
    {
        println!("Machine Timer Interrupt");

        return;
    } else if mcause.is_interrupt()
        && mcause.code() == riscv::interrupt::Interrupt::MachineExternal as _
    {
        println!("Machine External Interrupt");
    } else {
        println!("Unknown trap");
    }

    loop {}
}

// board_early_init
#[no_mangle]
unsafe extern "C" fn _early_init() {
    ptr::write_volatile(0x9110_8020 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8030 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8000 as *mut u32, 0x69);

    ptr::write_volatile(0x9110_0004 as *mut u32, 0x8019_9805);

    // SYSCTL_PWR_BASE_ADDRn
    ptr::write_volatile((0x91103000_u32 + 0x158) as *mut u32, 0x0);

    // disable all pmp
    asm!(
        "
        li t0, 0xffffffff
        csrw pmpaddr0, t0
        li t0, 0x1f
        csrw pmpcfg0, t0
    "
    );

    // performance settings
    if false {
        asm!(
            "
        la t0, 0x70013
        // MCOR
        csrw 0x7c2, t0
        la t0, 0xe0000009
        // MCCR2
        csrw 0x7c3, t0

        la t0, 0x11ff
        // MHCR
        csrw 0x7c1, t0

        la t0, 0x638000
        // MXSTATUS
        csrw 0x7c0, t0

        la t0, 0x6e30c
        // MHINT
        csrw 0x7c5, t0
        "
        );
    }

    {
        use riscv::register::*;

        mstatus::set_mie(); // enable global interrupt
        mstatus::set_sie(); // and supervisor interrupt
        mie::set_mext(); // and external interrupt
        mie::set_msoft(); // and software interrupt
        mie::set_mtimer(); // and timer interrupt

        mcounteren::set_cy(); // enable cycle counter
        mcounteren::set_tm(); // and time counter

        // FPU init
        mstatus::set_fs(mstatus::FS::Clean);
        mstatus::set_fs(mstatus::FS::Initial);
        asm!("csrwi fcsr, 0");
    }
}

// ASCII art of "Rust"
const BANNER: &str = include_str!("BANNER");

pub struct Console;

impl core::fmt::Write for Console {
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

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    UART0.thr().write(|w| w.set_thr(0x42));
    UART0.thr().write(|w| w.set_thr(0x42));
    UART0.thr().write(|w| w.set_thr(0x42));

    UART0.ier().write(|w| w.0 = 0);
    UART0.fcr().write(|w| {
        w.set_fifoe(false);
        w.set_xfifor(true);
    });

    let mut con = Console;

    UART0.thr().write(|w| w.set_thr(0x41));
    UART0.thr().write(|w| w.set_thr(0x41));
    UART0.thr().write(|w| w.set_thr(0x41));

    writeln!(con).unwrap();

    writeln!(con, "{}", BANNER).unwrap();

    writeln!(con, "Booting K230 using Rust ....").unwrap();

    ddr_init::board_ddr_init();

    writeln!(con, "DDR init done!").unwrap();

    let mstatus = riscv::register::mstatus::read();
    println!("mstatus: {:016x}", mstatus.bits());

    let mie = riscv::register::mie::read();
    println!("mie: {:016x}", mie.bits());

    let mip = riscv::register::mip::read();
    println!("mip: {:016x}", mip.bits());

    let misa = riscv::register::misa::read().unwrap();

    println!("misa: {:x}", misa.bits());
    write!(con, "  RV64").unwrap();
    for c in 'A'..='Z' {
        if misa.has_extension(c) {
            write!(con, "{}", c).unwrap();
        }
    }
    writeln!(con).unwrap();

    let mvendorid = riscv::register::mvendorid::read().unwrap();
    println!("mvendorid: {:x}", mvendorid.bits());

    let marchid = riscv::register::marchid::read().unwrap();
    println!("marchid: {:x}", marchid.bits());

    let mut cpuid0: u64;
    let mut cpuid1: u64;
    let mut cpuid2: u64;
    asm!("
        csrr {0}, 0xfc0
        csrr {1}, 0xfc0
        csrr {2}, 0xfc0
    ", out(reg) cpuid0, out(reg) cpuid1, out(reg) cpuid2);
    println!("cpuid: {:08x} {:08x} {:08x}", cpuid0, cpuid1, cpuid2);

    // read csr 0xfc1 mapbaddr, p
    let mut mapbaddr: u64;
    asm!("csrr {0}, 0xfc1", out(reg) mapbaddr);
    println!("PLIC base: 0x{:016x}", mapbaddr);

    let r = pac::GPIO0.config_reg1().read();
    println!(
        "GPIO0 config_reg1: num_ports={} debounce={} PA_hw_ctl={} PA_single_ctl={}",
        r.num_ports() + 1,
        r.debounce(),
        r.hw_port(0),
        r.port_single_ctl(0)
    );
    let r = pac::GPIO0.config_reg2().read();
    println!("GPIO0 config_reg2: len(PA)={}", r.encoded_id_pwidth(0) + 1,);

    let r = pac::GPIO0.ver_id_code().read();
    println!("GPIO0 ver id=0x{:08x}", r);

    let mut delay = riscv::delay::McycleDelay::new(CPU0_CORE_CLK);

    loop {
        // delay.delay_ms(1000); panic!("fuck"); // - test trap

        //asm!("rdtime {0}", out(reg) time);
        //println!("mtime: {}", time);

        println!(
            "mtime:    L:{} H:{}",
            pac::CLINT.mtime().read(),
            pac::CLINT.mtimeh().read(),
        );

        let mcycle = riscv::register::mcycle::read64();
        println!("mcycle: {}", mcycle);

        delay.delay_ms(1000);

        pac::CLINT.msip(0).write(|w| w.set_msip(true));
    }
}

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    asm!(
        "
        la x31, 0x1f
        .word 0x00000000"
    );
    loop {}
}
