use core::{
    arch::{asm, global_asm},
    ptr,
};

use crate::println;

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

    call __pre_init

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
);

#[unsafe(link_section = ".trap")]
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
unsafe extern "C" fn _early_init() {
    // STC init
    unsafe {
        ptr::write_volatile(0x9110_8020 as *mut u32, 0x1);
        ptr::write_volatile(0x9110_8030 as *mut u32, 0x1);
        ptr::write_volatile(0x9110_8000 as *mut u32, 0x69);
    }

    // CMU?
    unsafe {
        ptr::write_volatile(0x9110_0004 as *mut u32, 0x8019_9805);
    }

    // SYSCTL_PWR_BASE_ADDRn
    // ptr::write_volatile((0x91103000_u32 + 0x158) as *mut u32, 0x0);
    pac::PWR.pmu_pwr_lpi_ctl().write_value(0);

    // disable all pmp
    unsafe {
        asm!(
            "
            li t0, 0xffffffff
            csrw pmpaddr0, t0
            li t0, 0x1f
            csrw pmpcfg0, t0
        "
        );
    }

    // performance settings
    if false {
        unsafe {
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
    }

    unsafe {
        use riscv::register::*;

        mstatus::set_mie(); // enable global interrupt
        mstatus::set_sie(); // and supervisor interrupt
        mie::set_mext(); // and external interrupt
        mie::set_msoft(); // and software interrupt
        mie::set_mtimer(); // and timer interrupt

        mcounteren::set_cy(); // enable cycle counter
        mcounteren::set_tm(); // and time counter

        // FPU init
        mstatus::set_fs(mstatus::FS::Initial);
        asm!("csrwi fcsr, 0");
    }

    // UART0, memory, spl
    board_init();
}

pub fn board_init() {
    use pac::UART0;
    // UART init
    // UART is inited by BootROM, so we just disable FIFO and interrupt
    UART0.ier().write(|w| w.0 = 0);
    UART0.fcr().write(|w| {
        w.set_fifoe(false);
        w.set_xfifor(true);
        w.set_rfifor(true);
    });

    println!("DDR init ...");

    unsafe {
        spl_device_disable();

        #[cfg(feature = "init_ddr")]
        {
            use crate::ddr_init;
            ddr_init::ddr_init_training();
        }
    }

    println!("DDR init done!");
}

unsafe fn spl_device_disable() {
    unsafe {
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
    }

    // check disable status
    unsafe {
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
}

#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        #[unsafe(export_name = "_start_rust")]
        pub extern "C" fn __impl_start_rust() -> ! {
            static BOOTINFO: BootInfo = BootInfo {
                debug_console: $crate::console::Console,
            };
            // validate the signature of the program entry point
            let f: fn(&'static $crate::bootinfo::BootInfo) -> ! = $path;

            f(&BOOTINFO)
        }
    };
}
