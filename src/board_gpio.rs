use core::ptr;

pub const LED_R: u8 = 65;
pub const LED_G: u8 = 66;
pub const LED_B: u8 = 71;

const PMU_BASE: usize = 0x9100_0000;
const PMU_INT0_TO_CTL: usize = 0x40;
const PMU_INT1_TO_CTL: usize = 0x44;
const PMU_INT0_TO_CPU: usize = 0x48;
const PMU_INT_DETECT_EN: usize = 0x4c;
const PMU_INT_DETECT_TYP: usize = 0x50;
const PMU_INT_DETECT_CLR: usize = 0x54;
const PMU_SYSCTRL: usize = 0x78;
const PMU_OUT_DELAY_SET: usize = 0x7c;
const PMU_IO_CFG0: usize = 0x80;
const PMU_NORMAL_TIMER_VAL: usize = 0xa0;
const PMU_OUT_EVENT_CTRL: usize = 0xa4;
const PMU_OUT_LOGIC_CTRL: usize = 0xa8;
const PMU_INT_STATE: usize = 0xac;

const PMU_IO_SEL_MASK: u32 = 0x7 << 11;
const PMU_IO_SEL_GPIO: u32 = 0x1 << 11;
const PMU_IO_IE: u32 = 1 << 8;
const PMU_IO_OE: u32 = 1 << 7;
const PMU_IO_PU: u32 = 1 << 6;
const PMU_IO_PD: u32 = 1 << 5;
const PMU_IO_DS_MASK: u32 = 0x7 << 1;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GpioError {
    InvalidPin,
    PmuPinRequiresAllowlist,
}

pub struct PinDump {
    pub pin: u8,
    pub group: u8,
    pub port: u8,
    pub bit: u8,
    pub iomux: u32,
    pub dr: u32,
    pub ddr: u32,
    pub ctl: u32,
    pub ext: u32,
}

pub fn set_gpio(pin: u8, high: bool) -> Result<(), GpioError> {
    if pin >= 64 {
        return Err(GpioError::PmuPinRequiresAllowlist);
    }

    configure_normal_gpio_output(pin)?;
    write_gpio_bit(pin, high)
}

pub fn set_pmu_led_gpio(pin: u8, high: bool) -> Result<(), GpioError> {
    if !is_led_pin(pin) {
        return Err(GpioError::PmuPinRequiresAllowlist);
    }

    configure_pmu_gpio_output(pin)?;
    write_gpio_bit(pin, high)
}

pub fn read_gpio(pin: u8) -> Result<bool, GpioError> {
    let (_, port, bit) = gpio_group_port_bit(pin)?;
    let ext = gpio_ext_port(pin, port);
    Ok((ext & (1u32 << bit)) != 0)
}

pub fn dump_pin(pin: u8) -> Result<PinDump, GpioError> {
    let (group, port, bit) = gpio_group_port_bit(pin)?;
    let iomux = if pin < 64 {
        pac::IOMUX.pad(pin as usize).read().0
    } else {
        read_pmu(io_cfg_offset(pin)?)
    };

    let gpio = gpio_block(pin)?;
    let port_regs = gpio.swport(port as usize);

    Ok(PinDump {
        pin,
        group,
        port,
        bit,
        iomux,
        dr: port_regs.dr().read(),
        ddr: port_regs.ddr().read(),
        ctl: port_regs.ctl().read(),
        ext: gpio.ext_port(port as usize).read(),
    })
}

pub fn set_rgb(red: bool, green: bool, blue: bool) -> Result<(), GpioError> {
    set_pmu_led_gpio(LED_R, red)?;
    set_pmu_led_gpio(LED_G, green)?;
    set_pmu_led_gpio(LED_B, blue)?;
    Ok(())
}

pub fn set_pmu_out1(high: bool) {
    configure_pmu_out1();
    update_pmu(PMU_SYSCTRL, 1u32 << 1, (high as u32) << 1);
}

pub fn pmu_out1_level() -> bool {
    (read_pmu(PMU_SYSCTRL) & (1u32 << 1)) != 0
}

pub fn dump_pmu() {
    crate::println!("PMU base 0x{:08x}", PMU_BASE);
    print_pmu_reg("INT0_TO_CTL", PMU_INT0_TO_CTL);
    print_pmu_reg("INT1_TO_CTL", PMU_INT1_TO_CTL);
    print_pmu_reg("INT0_TO_CPU", PMU_INT0_TO_CPU);
    print_pmu_reg("INT_DETECT_EN", PMU_INT_DETECT_EN);
    print_pmu_reg("INT_DETECT_TYP", PMU_INT_DETECT_TYP);
    print_pmu_reg("INT_DETECT_CLR", PMU_INT_DETECT_CLR);
    print_pmu_reg("SYSCTRL", PMU_SYSCTRL);
    print_pmu_reg("OUT_DELAY_SET", PMU_OUT_DELAY_SET);
    print_pmu_reg("NORMAL_TIMER_VAL", PMU_NORMAL_TIMER_VAL);
    print_pmu_reg("OUT_EVENT_CTRL", PMU_OUT_EVENT_CTRL);
    print_pmu_reg("OUT_LOGIC_CTRL", PMU_OUT_LOGIC_CTRL);
    print_pmu_reg("INT_STATE", PMU_INT_STATE);

    for pin in 64..=71 {
        if let Ok(offset) = io_cfg_offset(pin) {
            crate::println!("PMU_IO_CFG_{}: 0x{:08x}", pin - 64, read_pmu(offset));
        }
    }

    crate::println!(
        "PWR PMU_PWR_LPI_CTL: 0x{:08x}",
        pac::PWR.pmu_pwr_lpi_ctl().read()
    );
    crate::println!(
        "PWR SSYS_CTL_GPIO_CTL: 0x{:08x}",
        pac::PWR.ssys_ctl_gpio_ctl().read()
    );
    crate::println!(
        "PWR SSYS_CTL_GPIO_EN0: 0x{:08x}",
        pac::PWR.ssys_ctl_gpio_en0().read()
    );
    crate::println!(
        "PWR SSYS_CTL_GPIO_EN1: 0x{:08x}",
        pac::PWR.ssys_ctl_gpio_en1().read()
    );
}

fn configure_normal_gpio_output(pin: u8) -> Result<(), GpioError> {
    if pin >= 64 {
        return Err(GpioError::PmuPinRequiresAllowlist);
    }

    pac::IOMUX.pad(pin as usize).modify(|w| {
        w.set_sel(0);
        w.set_oe(true);
        w.set_ie(false);
        w.set_pu(false);
        w.set_pd(false);
        w.set_ds(7);
    });

    configure_gpio_port_output(pin)
}

fn configure_pmu_gpio_output(pin: u8) -> Result<(), GpioError> {
    let offset = io_cfg_offset(pin)?;
    let mut cfg = read_pmu(offset);
    cfg &= !(PMU_IO_SEL_MASK | PMU_IO_IE | PMU_IO_PU | PMU_IO_PD | PMU_IO_DS_MASK);
    cfg |= PMU_IO_SEL_GPIO | PMU_IO_OE | (0x7 << 1);
    write_pmu(offset, cfg);

    configure_gpio_port_output(pin)
}

fn configure_gpio_port_output(pin: u8) -> Result<(), GpioError> {
    let (_, port, bit) = gpio_group_port_bit(pin)?;
    let gpio = gpio_block(pin)?;
    let port_regs = gpio.swport(port as usize);
    let mask = 1u32 << bit;

    port_regs.ctl().modify(|w| *w &= !mask);
    port_regs.ddr().modify(|w| *w |= mask);
    Ok(())
}

fn write_gpio_bit(pin: u8, high: bool) -> Result<(), GpioError> {
    let (_, port, bit) = gpio_group_port_bit(pin)?;
    let gpio = gpio_block(pin)?;
    let port_regs = gpio.swport(port as usize);
    let mask = 1u32 << bit;

    port_regs.dr().modify(|w| {
        if high {
            *w |= mask;
        } else {
            *w &= !mask;
        }
    });
    Ok(())
}

fn gpio_ext_port(pin: u8, port: u8) -> u32 {
    gpio_block(pin)
        .map(|gpio| gpio.ext_port(port as usize).read())
        .unwrap_or(0)
}

fn gpio_block(pin: u8) -> Result<pac::gpio::Gpio, GpioError> {
    match pin {
        0..=31 => Ok(pac::GPIO0),
        32..=71 => Ok(pac::GPIO1),
        _ => Err(GpioError::InvalidPin),
    }
}

fn gpio_group_port_bit(pin: u8) -> Result<(u8, u8, u8), GpioError> {
    // PMU GPIO64-71 are exposed as GPIO group2 in the TRM and are wired to
    // GPIO1 port1 in the DW_apb_gpio register block.
    match pin {
        0..=31 => Ok((0, 0, pin)),
        32..=63 => Ok((1, 0, pin - 32)),
        64..=71 => Ok((2, 1, pin - 64)),
        _ => Err(GpioError::InvalidPin),
    }
}

fn io_cfg_offset(pin: u8) -> Result<usize, GpioError> {
    match pin {
        64..=71 => Ok(PMU_IO_CFG0 + ((pin - 64) as usize * 4)),
        _ => Err(GpioError::InvalidPin),
    }
}

fn configure_pmu_out1() {
    let mut cfg = read_pmu(PMU_IO_CFG0 + 7 * 4);
    cfg &= !(PMU_IO_SEL_MASK | PMU_IO_IE | PMU_IO_PU | PMU_IO_PD | PMU_IO_DS_MASK);
    cfg |= (0x2 << 11) | PMU_IO_OE | (0x7 << 1);
    write_pmu(PMU_IO_CFG0 + 7 * 4, cfg);

    update_pmu(PMU_OUT_LOGIC_CTRL, 0b111 << 1, 0b100 << 1);
}

fn is_led_pin(pin: u8) -> bool {
    matches!(pin, LED_R | LED_G | LED_B)
}

fn print_pmu_reg(name: &str, offset: usize) {
    crate::println!(
        "PMU_{:<16} @ +0x{:03x}: 0x{:08x}",
        name,
        offset,
        read_pmu(offset)
    );
}

fn read_pmu(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((PMU_BASE + offset) as *const u32) }
}

fn write_pmu(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((PMU_BASE + offset) as *mut u32, value) }
}

fn update_pmu(offset: usize, mask: u32, value: u32) {
    let next = (read_pmu(offset) & !mask) | (value & mask);
    write_pmu(offset, next);
}
