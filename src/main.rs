#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use core::{arch::asm, ptr};
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

pub mod boot;
pub mod commands;
#[allow(unused)]
pub mod ddr_init;
pub mod readline;
pub mod serial;

pub mod rt;

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

// ASCII art of "Rust"
const BANNER: &str = include_str!("BANNER");

#[derive(Debug)]
pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
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

pub fn getchar() -> u8 {
    unsafe {
        while !UART0.lsr().read().dr() {
            asm!("nop");
        }

        UART0.rbr().read().rbr()
    }
}

pub fn putc(c: u8) {
    unsafe {
        while !UART0.lsr().read().thre() {
            asm!("nop");
        }

        UART0.thr().write(|w| w.set_thr(c));
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
    println!("DDR init ...");

    // spl_board_init_f
    spl_device_disable();
    ddr_init::ddr_init_training();

    println!("DDR init done!");
}

// init UART3
fn uart_init() {
    let clk_in = 50_000_000;
    let r = pac::UART3;

    // todo: enable apb clock

    // set io mux
    unsafe {
        ptr::write_volatile(0x911050c8 as *mut u32, 0x00000a8f); // UART3_TX. GPIO50
        ptr::write_volatile(0x911050cc as *mut u32, 0x80000bd0); // UART3_RX
    }

    let baud = 115200;
    let div = clk_in / (16 * baud);

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

fn tsensor_init() {
    use pac::TSENSOR;

    TSENSOR.tsen_w().write(|w| {
        w.set_ts_conv_mode(true); // continuous mode
        w.set_ts_test_en(true);
    });
    TSENSOR.tsen_w().modify(|w| {
        w.set_ts_en(true);
    });
}

fn blinky() {
    // RGB LED of LCKFB
    // - R: GPIO62
    // - G: GPIO20
    // - B: GPIO63
    use pac::{GPIO0, GPIO1, IOMUX};

    IOMUX.pad(20).modify(|w| w.set_sel(0));
    IOMUX.pad(62).modify(|w| w.set_sel(0));
    IOMUX.pad(63).modify(|w| w.set_sel(0));

    GPIO0.swport(0).ddr().modify(|w| *w |= 1 << 20);
    GPIO1.swport(0).ddr().modify(|w| *w |= 1 << 30);
    GPIO1.swport(0).ddr().modify(|w| *w |= 1 << 31);

    println!("dr =  {:08x}", GPIO0.swport(0).dr().read());
    println!("ddr = {:08x}", GPIO0.swport(0).ddr().read());

    loop {
        GPIO0.swport(0).dr().modify(|w| *w ^= 1 << 20);
        GPIO1.swport(0).dr().modify(|w| *w ^= 1 << 31);

        riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(1000);

        println!("blinky");
        println!("dr =  {:08x}", GPIO0.swport(0).dr().read());
        println!("ddr = {:08x}", GPIO0.swport(0).ddr().read());
    }
}

fn buzzer() {
    // GPIO43 - PWM1
    use pac::{IOMUX, PWM0};

    // PCLK, PWM use APB clock to program registers as well as to generate waveforms. The default frequency is 100MHz.
    // const PWM_CLOCK_IN: u32 = 100_000_000;

    IOMUX.pad(43).modify(|w| {
        w.set_sel(2); // PWM = 2
        w.set_oe(true);
        w.set_ds(7);
    });

    // Calc:
    // scale = 2
    // period = 0x5000
    // duty = period / 2 = 0x2800
    // 100_000_000 / (1 << 2) / 0x5000
    // duty = 0x5000 / 2

    PWM0.pwmcfg().modify(|w| {
        w.set_zerocomp(true);
        w.set_scale(2);
    });

    // CMP_max = 0xFFFF. 64bit
    PWM0.pwmcmp(0).write(|w| w.0 = 0x5000);
    let duty = 0x2800;

    PWM0.pwmcmp(2).modify(|w| w.0 = duty);

    // enable
    PWM0.pwmcfg().modify(|w| w.set_enalways(true));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(100);

    PWM0.pwmcfg().modify(|w| w.set_enalways(false));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(100);

    PWM0.pwmcfg().modify(|w| w.set_enalways(true));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(100);

    PWM0.pwmcfg().modify(|w| w.set_enalways(false));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(100);

    PWM0.pwmcfg().modify(|w| w.set_enalways(true));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(100);

    PWM0.pwmcfg().modify(|w| w.set_enalways(false));
}

// require buzzer init
fn beep() {
    use pac::PWM0;
    PWM0.pwmcfg().modify(|w| w.set_enalways(true));
    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(50);

    PWM0.pwmcfg().modify(|w| w.set_enalways(false));
}

fn shell_repl() {
    use noline::builder::EditorBuilder;
    use noline::error::NolineError;

    let mut buffer = [0; 1024];
    let mut history = [0; 1024];
    // noline doesn't support color prompt
    // const PROPMT: &str = "\x1b[32;1mK230\x1b[0m> ";
    const PROPMT: &str = "K230> ";

    let mut editor = EditorBuilder::from_slice(&mut buffer)
        .with_slice_history(&mut history)
        .build_sync(&mut Console)
        .unwrap();
    loop {
        match editor.readline(PROPMT, &mut Console) {
            Ok(s) => {
                if s.len() > 0 {
                    beep();
                    commands::handle_command_line(s);
                } else {
                    println!("");
                }
            }
            Err(err) => {
                let error = match err {
                    NolineError::IoError(_) => "IoError",
                    NolineError::ParserError => "ParserError",
                    NolineError::Aborted => "Aborted",
                };
                println!("Error: {}\r", error);
            }
        }
    }
}

#[no_mangle]
unsafe extern "C" fn _start_rust() -> ! {
    board_init();

    commands::cpuid();

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

    tsensor_init();
    buzzer();

    uart_init();
    let mut i = 0;
    while i < 10 {
        if pac::UART3.lsr().read().thre() {
            pac::UART3.thr().write(|w| w.set_thr(b'A'));
            i += 1;
        } else {
        }
    }

    // blinky();

    boot::litex_term_serial_boot();

    shell_repl();

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
    println!("PANIC: {}", _info);

    riscv::delay::McycleDelay::new(CPU0_CORE_CLK).delay_ms(1000);

    loop {}
}
