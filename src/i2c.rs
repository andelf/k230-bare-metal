use pac::i2c::I2c;

const I2C0_SCL_BIT: u32 = 1 << 16;
const I2C0_SDA_BIT: u32 = 1 << 17;

const IC_CON_MASTER: u32 = 1 << 0;
const IC_CON_SPEED_STANDARD: u32 = 1 << 1;
const IC_CON_RESTART_EN: u32 = 1 << 5;
const IC_CON_SLAVE_DISABLE: u32 = 1 << 6;

const IC_DATA_CMD_READ: u32 = 1 << 8;
const IC_DATA_CMD_STOP: u32 = 1 << 9;
const IC_DATA_CMD_RESTART: u32 = 1 << 10;

const IC_RAW_TX_ABRT: u32 = 1 << 6;

const IC_STATUS_TFNF: u32 = 1 << 1;
const IC_STATUS_TFE: u32 = 1 << 2;
const IC_STATUS_RFNE: u32 = 1 << 3;
const IC_STATUS_MST_ACTIVITY: u32 = 1 << 5;

const WAIT_TIMEOUT_US: u64 = 5_000;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidBus,
    InvalidAddress,
    Timeout,
    TxAbort(u32),
}

pub struct I2cMaster {
    regs: I2c,
    ic_clk_hz: u32,
}

impl I2cMaster {
    pub const fn new(regs: I2c, ic_clk_hz: u32) -> Self {
        Self { regs, ic_clk_hz }
    }

    pub fn init_standard(&self, bus_hz: u32) -> Result<(), Error> {
        self.disable()?;

        self.regs.con().write_value(
            IC_CON_MASTER | IC_CON_SPEED_STANDARD | IC_CON_RESTART_EN | IC_CON_SLAVE_DISABLE,
        );
        self.regs.intr_mask().write_value(0);
        self.regs.rx_tl().write_value(0);
        self.regs.tx_tl().write_value(0);

        let half_period = (self.ic_clk_hz / bus_hz / 2).max(8);
        self.regs.ss_scl_hcnt().write_value(half_period);
        self.regs.ss_scl_lcnt().write_value(half_period);
        self.regs.sda_hold().write_value(half_period / 4);

        Ok(())
    }

    pub fn probe(&self, addr: u8) -> Result<(), Error> {
        let mut byte = [0u8; 1];
        self.read(addr, &mut byte)
    }

    pub fn read_mem8(&self, addr: u8, reg: u8, out: &mut [u8]) -> Result<(), Error> {
        if out.is_empty() {
            return Ok(());
        }
        self.begin(addr)?;
        self.write_cmd(reg as u32)?;

        for index in 0..out.len() {
            let mut cmd = IC_DATA_CMD_READ;
            if index == 0 {
                cmd |= IC_DATA_CMD_RESTART;
            }
            if index + 1 == out.len() {
                cmd |= IC_DATA_CMD_STOP;
            }
            self.write_cmd(cmd)?;
        }

        for byte in out.iter_mut() {
            self.wait_status(IC_STATUS_RFNE, IC_STATUS_RFNE)?;
            *byte = (self.regs.data_cmd().read() & 0xff) as u8;
            self.check_abort()?;
        }

        self.wait_idle()
    }

    pub fn write_mem8(&self, addr: u8, reg: u8, data: &[u8]) -> Result<(), Error> {
        self.begin(addr)?;
        self.write_cmd(reg as u32)?;
        for (index, byte) in data.iter().enumerate() {
            let mut cmd = *byte as u32;
            if index + 1 == data.len() {
                cmd |= IC_DATA_CMD_STOP;
            }
            self.write_cmd(cmd)?;
        }
        self.wait_idle()
    }

    pub fn read(&self, addr: u8, out: &mut [u8]) -> Result<(), Error> {
        if out.is_empty() {
            return Ok(());
        }
        self.begin(addr)?;
        for index in 0..out.len() {
            let mut cmd = IC_DATA_CMD_READ;
            if index + 1 == out.len() {
                cmd |= IC_DATA_CMD_STOP;
            }
            self.write_cmd(cmd)?;
        }
        for byte in out.iter_mut() {
            self.wait_status(IC_STATUS_RFNE, IC_STATUS_RFNE)?;
            *byte = (self.regs.data_cmd().read() & 0xff) as u8;
            self.check_abort()?;
        }
        self.wait_idle()
    }

    pub fn dump(&self) {
        crate::println!(
            "LS_CLKEN_CFG0       0x{:08x}",
            pac::CMU.ls_clken_cfg0().read()
        );
        crate::println!(
            "UART_I2C_CLKDIV_CFG 0x{:08x}",
            pac::CMU.uart_i2c_clkdiv_cfg().read()
        );
        crate::println!("IC_CON             0x{:08x}", self.regs.con().read());
        crate::println!("IC_TAR             0x{:08x}", self.regs.tar().read());
        crate::println!("IC_ENABLE          0x{:08x}", self.regs.enable().read());
        crate::println!("IC_STATUS          0x{:08x}", self.regs.status().read());
        crate::println!(
            "IC_RAW_INTR_STAT   0x{:08x}",
            self.regs.raw_intr_stat().read()
        );
        crate::println!("IC_TXFLR           0x{:08x}", self.regs.txflr().read());
        crate::println!("IC_RXFLR           0x{:08x}", self.regs.rxflr().read());
        crate::println!(
            "IC_TX_ABRT_SOURCE  0x{:08x}",
            self.regs.tx_abrt_source().read()
        );
        crate::println!(
            "IC_ENABLE_STATUS   0x{:08x}",
            self.regs.enable_status().read()
        );
        crate::println!(
            "IC_COMP_PARAM_1    0x{:08x}",
            self.regs.comp_param_1().read()
        );
        crate::println!(
            "IC_COMP_VERSION    0x{:08x}",
            self.regs.comp_version().read()
        );
        crate::println!("IC_COMP_TYPE       0x{:08x}", self.regs.comp_type().read());
    }

    fn begin(&self, addr: u8) -> Result<(), Error> {
        if addr > 0x7f {
            return Err(Error::InvalidAddress);
        }
        self.disable()?;
        self.regs.tar().write_value(addr as u32);
        let _ = self.regs.clr_intr().read();
        self.enable()
    }

    fn write_cmd(&self, cmd: u32) -> Result<(), Error> {
        self.wait_status(IC_STATUS_TFNF, IC_STATUS_TFNF)?;
        self.regs.data_cmd().write_value(cmd);
        self.check_abort()
    }

    fn enable(&self) -> Result<(), Error> {
        self.regs.enable().write_value(1);
        self.wait_enable_status(true)
    }

    fn disable(&self) -> Result<(), Error> {
        self.regs.enable().write_value(0);
        self.wait_enable_status(false)
    }

    fn wait_enable_status(&self, enabled: bool) -> Result<(), Error> {
        let deadline = crate::time::deadline_after_us(WAIT_TIMEOUT_US);
        while !deadline.expired() {
            let is_enabled = (self.regs.enable_status().read() & 1) != 0;
            if is_enabled == enabled {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        self.check_abort()?;
        Err(Error::Timeout)
    }

    fn wait_status(&self, mask: u32, value: u32) -> Result<(), Error> {
        let deadline = crate::time::deadline_after_us(WAIT_TIMEOUT_US);
        while !deadline.expired() {
            self.check_abort()?;
            if (self.regs.status().read() & mask) == value {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        self.check_abort()?;
        Err(Error::Timeout)
    }

    fn wait_idle(&self) -> Result<(), Error> {
        let deadline = crate::time::deadline_after_us(WAIT_TIMEOUT_US);
        while !deadline.expired() {
            self.check_abort()?;
            let status = self.regs.status().read();
            if (status & (IC_STATUS_TFE | IC_STATUS_MST_ACTIVITY)) == IC_STATUS_TFE {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        self.check_abort()?;
        Err(Error::Timeout)
    }

    fn check_abort(&self) -> Result<(), Error> {
        if (self.regs.raw_intr_stat().read() & IC_RAW_TX_ABRT) != 0 {
            let source = self.regs.tx_abrt_source().read();
            let _ = self.regs.clr_tx_abrt().read();
            return Err(Error::TxAbort(source));
        }
        Ok(())
    }
}

pub fn bus(id: u8) -> Result<I2cMaster, Error> {
    enable_bus_clock(id)?;

    match id {
        0 => Ok(I2cMaster::new(pac::I2C0, crate::I2C0_CLK)),
        1 => Ok(I2cMaster::new(pac::I2C1, crate::I2C1_CLK)),
        2 => Ok(I2cMaster::new(pac::I2C2, crate::I2C2_CLK)),
        3 => Ok(I2cMaster::new(pac::I2C3, crate::I2C3_CLK)),
        4 => Ok(I2cMaster::new(pac::I2C4, crate::I2C4_CLK)),
        _ => Err(Error::InvalidBus),
    }
}

fn enable_bus_clock(id: u8) -> Result<(), Error> {
    if id > 4 {
        return Err(Error::InvalidBus);
    }

    let pclk_bit = 6 + u32::from(id);
    let core_bit = 21 + u32::from(id);
    pac::CMU
        .ls_clken_cfg0()
        .modify(|w| *w |= (1 << pclk_bit) | (1 << core_bit));

    Ok(())
}

pub fn init_i2c0_pins_48_49() {
    pac::IOMUX.pad(48).modify(|w| {
        w.set_sel(3);
        w.set_oe(true);
        w.set_ie(true);
        w.set_pu(true);
        w.set_pd(false);
        w.set_ds(7);
        w.set_st(true);
    });
    pac::IOMUX.pad(49).modify(|w| {
        w.set_sel(3);
        w.set_oe(true);
        w.set_ie(true);
        w.set_pu(true);
        w.set_pd(false);
        w.set_ds(7);
        w.set_st(true);
    });
}

pub fn bitbang_i2c0_levels() {
    init_i2c0_gpio_open_drain();
    bb_release(I2C0_SCL_BIT | I2C0_SDA_BIT);
    bb_delay();
    let ext = bb_ext();
    crate::println!(
        "I2C0 GPIO levels: SCL={} SDA={} ext=0x{:08x}",
        ((ext & I2C0_SCL_BIT) != 0) as u8,
        ((ext & I2C0_SDA_BIT) != 0) as u8,
        ext
    );
}

pub fn bitbang_i2c0_scan() {
    init_i2c0_gpio_open_drain();
    bb_recover();

    crate::print!("I2C0 bitbang devices:");
    let mut found = false;
    for addr in 0x08..=0x77 {
        bb_start();
        let ack = bb_write_byte(addr << 1);
        bb_stop();
        if ack {
            found = true;
            crate::print!(" 0x{:02x}", addr);
        }
    }
    if found {
        crate::println!();
    } else {
        crate::println!(" none");
    }
}

pub fn bitbang_i2c0_bme680_id() {
    init_i2c0_gpio_open_drain();
    bb_recover();

    let mut id = 0;
    if bitbang_read_mem8(0x77, 0xd0, &mut id) {
        crate::println!("BME680 bitbang addr=0x77 chip_id=0x{:02x}", id);
    } else if bitbang_read_mem8(0x76, 0xd0, &mut id) {
        crate::println!("BME680 bitbang addr=0x76 chip_id=0x{:02x}", id);
    } else {
        crate::println!("BME680 bitbang read failed");
    }
}

fn bitbang_read_mem8(addr: u8, reg: u8, out: &mut u8) -> bool {
    bb_start();
    if !bb_write_byte(addr << 1) {
        bb_stop();
        return false;
    }
    if !bb_write_byte(reg) {
        bb_stop();
        return false;
    }
    bb_start();
    if !bb_write_byte((addr << 1) | 1) {
        bb_stop();
        return false;
    }
    *out = bb_read_byte(false);
    bb_stop();
    true
}

fn init_i2c0_gpio_open_drain() {
    for pin in [48usize, 49] {
        pac::IOMUX.pad(pin).modify(|w| {
            w.set_sel(0);
            w.set_oe(true);
            w.set_ie(true);
            w.set_pu(true);
            w.set_pd(false);
            w.set_ds(7);
            w.set_st(true);
        });
    }

    let port = pac::GPIO1.swport(0);
    let mask = I2C0_SCL_BIT | I2C0_SDA_BIT;
    port.ctl().modify(|w| *w &= !mask);
    port.dr().modify(|w| *w &= !mask);
    port.ddr().modify(|w| *w &= !mask);
}

fn bb_recover() {
    bb_release(I2C0_SCL_BIT | I2C0_SDA_BIT);
    for _ in 0..9 {
        bb_delay();
        bb_drive_low(I2C0_SCL_BIT);
        bb_delay();
        bb_release(I2C0_SCL_BIT);
    }
    bb_stop();
}

fn bb_start() {
    bb_release(I2C0_SCL_BIT | I2C0_SDA_BIT);
    bb_delay();
    bb_drive_low(I2C0_SDA_BIT);
    bb_delay();
    bb_drive_low(I2C0_SCL_BIT);
}

fn bb_stop() {
    bb_drive_low(I2C0_SDA_BIT);
    bb_delay();
    bb_release(I2C0_SCL_BIT);
    bb_delay();
    bb_release(I2C0_SDA_BIT);
    bb_delay();
}

fn bb_write_byte(byte: u8) -> bool {
    for bit in (0..8).rev() {
        if (byte & (1 << bit)) != 0 {
            bb_release(I2C0_SDA_BIT);
        } else {
            bb_drive_low(I2C0_SDA_BIT);
        }
        bb_delay();
        bb_clock_high();
        bb_drive_low(I2C0_SCL_BIT);
    }

    bb_release(I2C0_SDA_BIT);
    bb_delay();
    bb_release(I2C0_SCL_BIT);
    bb_delay();
    let ack = (bb_ext() & I2C0_SDA_BIT) == 0;
    bb_drive_low(I2C0_SCL_BIT);
    ack
}

fn bb_read_byte(ack: bool) -> u8 {
    let mut byte = 0;
    bb_release(I2C0_SDA_BIT);
    for _ in 0..8 {
        byte <<= 1;
        bb_delay();
        bb_release(I2C0_SCL_BIT);
        bb_delay();
        if (bb_ext() & I2C0_SDA_BIT) != 0 {
            byte |= 1;
        }
        bb_drive_low(I2C0_SCL_BIT);
    }

    if ack {
        bb_drive_low(I2C0_SDA_BIT);
    } else {
        bb_release(I2C0_SDA_BIT);
    }
    bb_delay();
    bb_clock_high();
    bb_drive_low(I2C0_SCL_BIT);
    bb_release(I2C0_SDA_BIT);
    byte
}

fn bb_clock_high() {
    bb_release(I2C0_SCL_BIT);
    bb_delay();
}

fn bb_drive_low(mask: u32) {
    let port = pac::GPIO1.swport(0);
    port.dr().modify(|w| *w &= !mask);
    port.ddr().modify(|w| *w |= mask);
}

fn bb_release(mask: u32) {
    pac::GPIO1.swport(0).ddr().modify(|w| *w &= !mask);
}

fn bb_ext() -> u32 {
    pac::GPIO1.ext_port(0).read()
}

fn bb_delay() {
    for _ in 0..4000 {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}
