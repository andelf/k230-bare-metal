#![no_std]
#![no_main]

use core::fmt::{self, Write as _};
use core::{
    arch::{asm, global_asm},
    ptr,
};
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
            writeln!(&mut crate::Console, "").unwrap();
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

//      la t1, __global_pointer$
// csrw mie, zero
/*
 la t0, 0x91400000
 li t1, 0x41
 sb t1, 0(t0)
 nop
 nop
 sb t1, 0(t0)
   .option push
  .option norelax
.option pop
 */

cfg_global_asm!(
    // no "c" here, the same as riscv-rt
    ".attribute arch, \"rv64im\"",
    ".section .start, \"ax\"
     .global _start
_start:
     la t1, __stack_start__
     addi sp, t1, -16

     csrr a0, mhartid
     lui a1, 0x88888
     mv t0, zero
     mv t1, zero
     csrr a2, misa

     la s0, 0x91400000
     addi s1, s0, 0x14

     li t0, 0x41
     sb t0, 0(s0)
     li t0, 0x41
     sb t0, 0(s0)
     li t0, 0x41
     sb t0, 0(s0)
     nop
     nop
     call board_early_init
     nop
     nop

    la x18, 0x91101024
    lw x19, 0(x18)
    la x20, 0x91100024
    lw x21, 0(x20)
    nop
    nop
    nop
    nop
    nop
    nop
    nop
1:
    csrr t0, mhartid
    bne t0, zero, 1b
    nop
    nop
    nop
    nop
    nop
    nop
    nop

//    .word 0x00000000

    call _start_rust
    //

     li gp, 0

     csrw mie, zero

     li t0, -16
     addiw t1, zero, 513
     slli t1, t1, 0x16
     addi t1, t1, -368
     and sp, t1, t0


     beqz t0, 1f
hart1:
    wfi
    j hart1

1:

    ",
    /*
    "la t1, __stack_safe__
     addi sp, t1, -16
     call __pre_init
    ",*/
    // set sp
    "la t1, __stack_start__
     addiw sp, t1, -16
    .word 0x00000000
     call _start_rust",
    "
1:
    j 1b
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

#[no_mangle]
unsafe extern "C" fn board_early_init() {
    ptr::write_volatile(0x9110_8020 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8030 as *mut u32, 0x1);
    ptr::write_volatile(0x9110_8000 as *mut u32, 0x69);

    ptr::write_volatile(0x9110_0004 as *mut u32, 0x8019_9805);

    // SYSCTL_PWR_BASE_ADDRn
    ptr::write_volatile((0x91103000_u32 + 0x158) as *mut u32, 0x0);

    // write 0x24484dff to csr pmpaddr0
    asm!(
        "
        li t0, 0xffffffff
        csrw pmpaddr0, t0
        li t0, 0x1f
        csrw pmpcfg0, t0
    "
    );

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
    //while UART0.lsr().read().dr() == false {
    //   asm!("nop");
    //}
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

    // asm!("    .word 0x00000000");

    //let addr = 0x9140_0000_u32;

    /*
    let mut uart = MmioUart8250::new(addr);
    // no init need
    uart.init(50_000_000, 115200);
    uart.disable_received_data_available_interrupt(); // must
    */

    //let mut uart = MmioUart8250::new(addr);

    writeln!(con, "{}", BANNER).unwrap();

    writeln!(con, "Booting K230 using Rust ....").unwrap();

    ddr_init::board_ddr_init();

    writeln!(con, "DDR init done!").unwrap();

    let misa = riscv::register::misa::read().unwrap();

    println!("misa: {:x}", misa.bits());
    write!(con, "  RV64").unwrap();
    for c in 'A'..='Z' {
        if misa.has_extension(c) {
            write!(con, "{}", c).unwrap();
        }
    }
    writeln!(con).unwrap();

    loop {
        for _ in 0..8000000 {
            asm!("nop");
        }

        let mcycle = riscv::register::mcycle::read64();

        writeln!(con, "mcycle: {}", mcycle).unwrap();

        // asm!(".word 0x00000000",);
        // uart.write_byte(b'B');
    }
}

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    asm!(
        "
    la x31, 0x1f
    .word 0x00000000",
    );
    loop {}
}
