#![no_std]

pub mod common;

pub mod clint;
pub mod plic;

#[path = "boot_v0.rs"]
pub mod boot;
#[path = "cmu_v0.rs"]
pub mod cmu;
#[path = "pwr_v0.rs"]
pub mod pwr;
#[path = "rmu_v0.rs"]
pub mod rmu;

#[path = "gpio_v0.rs"]
pub mod gpio;

#[path = "uart_dw.rs"]
pub mod uart;

#[path = "iomux_v0.rs"]
pub mod iomux;

pub const PLIC_BASE: usize = 0x0000000_f00000000;

pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr((PLIC_BASE) as *mut ()) };

pub const CLINT: clint::Clint =
    unsafe { clint::Clint::from_ptr((PLIC_BASE + 0x0400_0000) as *mut ()) };

pub const BOOT: boot::Boot = unsafe { boot::Boot::from_ptr(0x9110_2000 as *mut ()) };

pub const CMU: cmu::Cmu = unsafe { cmu::Cmu::from_ptr(0x9110_0000 as *mut ()) };

pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x9110_3000 as *mut ()) };

pub const RMU: rmu::Rmu = unsafe { rmu::Rmu::from_ptr(0x9110_1000 as *mut ()) };

pub const IOMUX: iomux::Iomux = unsafe { iomux::Iomux::from_ptr(0x9110_5000 as *mut ()) };
pub const PMU_IOMUX: iomux::Iomux = unsafe { iomux::Iomux::from_ptr(9100_0080 as *mut ()) };

pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x9140_B000 as *mut ()) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x9140_C000 as *mut ()) };

pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x9140_0000 as *mut ()) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x9140_1000 as *mut ()) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0x9140_2000 as *mut ()) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0x9140_3000 as *mut ()) };
pub const UART4: uart::Uart = unsafe { uart::Uart::from_ptr(0x9140_4000 as *mut ()) };
