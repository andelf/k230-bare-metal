#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::fmt::{self, Write as _};
use core::{
    arch::{asm, global_asm},
    ptr,
};
use embedded_hal::delay::DelayNs;
use pac::{PWR, UART0};

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
    // STC init
    ptr::write_volatile(0x9110_8020 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8030 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8000 as *mut u32, 0x69);

    // CMU?
    ptr::write_volatile(0x9110_0004 as *mut u32, 0x8019_9805);

    // SYSCTL_PWR_BASE_ADDRn
    // ptr::write_volatile((0x91103000_u32 + 0x158) as *mut u32, 0x0);
    PWR.pmu_pwr_lpi_ctl().write_value(0);

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
    if true {
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

unsafe fn spl_device_disable() {
    // disable ai power
    if ptr::read_volatile(0x9110302c as *const u32) & 0x2 != 0 {
        ptr::write_volatile(0x91103028 as *mut u32, 0x30001);
    }

    // disable vpu power
    if ptr::read_volatile(0x91103080 as *const u32) & 0x2 != 0 {
        ptr::write_volatile(0x9110307c as *mut u32, 0x30001);
    }

    // disable dpu power
    if ptr::read_volatile(0x9110310c as *const u32) & 0x2 != 0 {
        ptr::write_volatile(0x91103108 as *mut u32, 0x30001);
    }

    // disable disp power
    if ptr::read_volatile(0x91103040 as *const u32) & 0x2 != 0 {
        ptr::write_volatile(0x9110303c as *mut u32, 0x30001);
    }

    // check disable status
    let mut value = 1000000;
    while (!(ptr::read_volatile(0x9110302c as *const u32) & 0x1 != 0)
        || !(ptr::read_volatile(0x91103080 as *const u32) & 0x1 != 0)
        || !(ptr::read_volatile(0x9110310c as *const u32) & 0x1 != 0)
        || !(ptr::read_volatile(0x91103040 as *const u32) & 0x1 != 0))
        && value != 0
    {
        value -= 1;
    }

    // disable ai clk
    value = ptr::read_volatile(0x91100008 as *const u32);
    value &= !(1 << 0);
    ptr::write_volatile(0x91100008 as *mut u32, value);

    // disable vpu clk
    value = ptr::read_volatile(0x9110000c as *const u32);
    value &= !(1 << 0);
    ptr::write_volatile(0x9110000c as *mut u32, value);

    // disable dpu clk
    value = ptr::read_volatile(0x91100070 as *const u32);
    value &= !(1 << 0);
    ptr::write_volatile(0x91100070 as *mut u32, value);

    // disable mclk
    value = ptr::read_volatile(0x9110006c as *const u32);
    value &= !((1 << 0) | (1 << 1) | (1 << 2));
    ptr::write_volatile(0x9110006c as *mut u32, value);
}

unsafe fn board_init() {
    // UART init
    // UART is inited by BootROM, so we just disable FIFO and interrupt
    UART0.ier().write(|w| w.0 = 0);
    UART0.fcr().write(|w| {
        w.set_fifoe(false);
        w.set_xfifor(true);
        w.set_rfifor(true);
    });

    println!("\r\n");
    println!("{}", BANNER);
    println!("Booting K230 using Rust ....");

    // spl_board_init_f
    spl_device_disable();
    ddr_init::ddr_init_training();

    println!("DDR init done!");
}

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    board_init();

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
    println!("GPIO0 config_reg1: num_ports={}", r.num_ports() + 1,);
    let r = pac::GPIO0.config_reg2().read();
    println!(
        "GPIO0 config_reg2: len(PA)={} len(PB)={}",
        r.encoded_id_pwidth(0) + 1,
        r.encoded_id_pwidth(1) + 1
    );
    let r = pac::GPIO1.config_reg1().read();
    println!("GPIO1 config_reg1: num_ports={}", r.num_ports() + 1,);
    let r = pac::GPIO1.config_reg2().read();
    println!(
        "GPIO1 config_reg2: len(PA)={} len(PB)={}",
        r.encoded_id_pwidth(0) + 1,
        r.encoded_id_pwidth(1) + 1
    );

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
