//! Serial driver

/// Device identity
pub unsafe trait DevId {
    const ADDRESS: *const ();

    const REGS: pac::uart::Uart = unsafe { pac::uart::Uart::from_ptr(Self::ADDRESS as _) };
}

// <'a, Id, M: Mode>
pub struct Uart<'a, Id> {
    _marker: core::marker::PhantomData<&'a Id>,
}

impl<'a, Id: DevId> Uart<'a, Id> {
    pub fn new() -> Self {
        Uart {
            _marker: core::marker::PhantomData,
        }
    }

    pub fn init(&self, clock_in: u32, baud_rate: u32) {
        let r = Id::REGS;

        // let baud = 115200;
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

    pub fn putc(&self, c: u8) {
        let r = Id::REGS;

        while !r.lsr().read().thre() {}
        r.thr().write(|w| w.set_thr(c));
    }

    pub fn getchar(&self) -> u8 {
        let r = Id::REGS;

        while !r.lsr().read().dr() {}
        r.rbr().read().rbr()
    }
}

impl<Id: DevId> core::fmt::Write for Uart<'_, Id> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            self.putc(*c);
        }

        Ok(())
    }
}

impl<Id: DevId> embedded_io::ErrorType for Uart<'_, Id> {
    type Error = core::convert::Infallible;
}

impl<Id: DevId> embedded_io::Read for Uart<'_, Id> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        for c in buf.iter_mut() {
            *c = self.getchar();
        }

        Ok(buf.len())
    }
}

impl<Id: DevId> embedded_io::ReadReady for Uart<'_, Id> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        let r = Id::REGS;
        Ok(r.lsr().read().dr())
    }
}

impl<Id: DevId> embedded_io::Write for Uart<'_, Id> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for c in buf.iter() {
            self.putc(*c);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let r = Id::REGS;
        while !r.lsr().read().temt() {}

        Ok(())
    }
}

impl<Id: DevId> embedded_io::WriteReady for Uart<'_, Id> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        let r = Id::REGS;
        Ok(r.lsr().read().thre())
    }
}
