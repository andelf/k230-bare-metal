#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Module power control register. 0x9110_3000"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "cpu0 power control timer parameter"]
    #[inline(always)]
    pub const fn cpu0_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "cpu0 NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn cpu0_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "cpu0 power control"]
    #[inline(always)]
    pub const fn cpu0_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "cpu0 power status"]
    #[inline(always)]
    pub const fn cpu0_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "cpu1 power control timer parameter"]
    #[inline(always)]
    pub const fn cpu1_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "cpu1 NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn cpu1_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "cpu1 power control"]
    #[inline(always)]
    pub const fn cpu1_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "cpu1 power status"]
    #[inline(always)]
    pub const fn cpu1_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ai power control timer parameter"]
    #[inline(always)]
    pub const fn ai_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ai NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn ai_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "ai power control"]
    #[inline(always)]
    pub const fn ai_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ai power status"]
    #[inline(always)]
    pub const fn ai_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "disp power control timer parameter"]
    #[inline(always)]
    pub const fn disp_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "disp NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn disp_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "gpu LPI control timer parameter"]
    #[inline(always)]
    pub const fn disp_gpu_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "disp power control"]
    #[inline(always)]
    pub const fn disp_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "disp power status"]
    #[inline(always)]
    pub const fn disp_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "shrm power control timer parameter"]
    #[inline(always)]
    pub const fn shrm_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "shrm NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn shrm_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "shrm power control"]
    #[inline(always)]
    pub const fn shrm_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "shrm power status"]
    #[inline(always)]
    pub const fn shrm_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "vpu power control timer parameter"]
    #[inline(always)]
    pub const fn vpu_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "vpu NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn vpu_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "vpu qchannel control timer parameter"]
    #[inline(always)]
    pub const fn vpu_qch_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "vpu power control"]
    #[inline(always)]
    pub const fn vpu_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "vpu power status"]
    #[inline(always)]
    pub const fn vpu_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "MCTRL power control timer parameter"]
    #[inline(always)]
    pub const fn mctl_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "MCTRL NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn mctl_noc_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "MCTRL ACI LPI control timer parameter"]
    #[inline(always)]
    pub const fn mctl_axi_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "MCTRLI power control"]
    #[inline(always)]
    pub const fn mctl_pwr_lpi_ctl(
        self,
    ) -> crate::common::Reg<regs::MctlPwrLpiCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "MCTRLI clock switch"]
    #[inline(always)]
    pub const fn mctl_clock_switch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "MCTRLI power status"]
    #[inline(always)]
    pub const fn mctl_lpi_stat(self) -> crate::common::Reg<regs::MctlLpiStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "dpu power control timer parameter"]
    #[inline(always)]
    pub const fn dpu_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "dpu NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn dpu_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "dpu power control"]
    #[inline(always)]
    pub const fn dpu_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "dpu power status"]
    #[inline(always)]
    pub const fn dpu_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "hi power control timer parameter"]
    #[inline(always)]
    pub const fn hi_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "hi NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn hi_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "hi power control"]
    #[inline(always)]
    pub const fn hi_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "hi power status"]
    #[inline(always)]
    pub const fn hi_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "ls power control timer parameter"]
    #[inline(always)]
    pub const fn ls_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "ls NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn ls_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "ls power control"]
    #[inline(always)]
    pub const fn ls_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "ls power status"]
    #[inline(always)]
    pub const fn ls_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "sec power control timer parameter"]
    #[inline(always)]
    pub const fn sec_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "sec NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn sec_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "sec power control"]
    #[inline(always)]
    pub const fn sec_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "sec power status"]
    #[inline(always)]
    pub const fn sec_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "isp power control timer parameter"]
    #[inline(always)]
    pub const fn isp_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "isp NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn isp_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "isp power control"]
    #[inline(always)]
    pub const fn isp_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "isp power status"]
    #[inline(always)]
    pub const fn isp_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "PMU power control timer parameter"]
    #[inline(always)]
    pub const fn pmu_pwr_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "PMU NOC LPI control timer parameter"]
    #[inline(always)]
    pub const fn pmu_lpi_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "PMU power control"]
    #[inline(always)]
    pub const fn pmu_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "PMU power status"]
    #[inline(always)]
    pub const fn pmu_pwr_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "sram repair timer"]
    #[inline(always)]
    pub const fn sram0_repair_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "gpio wakeup control"]
    #[inline(always)]
    pub const fn ssys_ctl_gpio_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "gpio wakeup enable"]
    #[inline(always)]
    pub const fn ssys_ctl_gpio_en0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "gpio wakeup enable"]
    #[inline(always)]
    pub const fn ssys_ctl_gpio_en1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "cpu wait repaire time"]
    #[inline(always)]
    pub const fn cpu_repair_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
}
pub mod regs {
    #[doc = "Memory Controller LPI Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MctlLpiStat(pub u32);
    impl MctlLpiStat {
        #[doc = "Memory Controller Power Control State: 0x0: Work; 0x1: DisconnectNoC; 0x2: SelfRefresh; 0x3: Retention; 0x4: ResetOn; 0x5: EnableISO; 0x6: PowerOff; 0x7: BringUpPower; 0x8: DisableISO; 0x9: ResetClkOn; 0xa: ResetRecover; 0xb: ResetRemove; 0xc: MemctlInit; 0xd: ConnectNoC."]
        #[inline(always)]
        pub const fn mctl_pwr_stat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Memory Controller Power Control State: 0x0: Work; 0x1: DisconnectNoC; 0x2: SelfRefresh; 0x3: Retention; 0x4: ResetOn; 0x5: EnableISO; 0x6: PowerOff; 0x7: BringUpPower; 0x8: DisableISO; 0x9: ResetClkOn; 0xa: ResetRecover; 0xb: ResetRemove; 0xc: MemctlInit; 0xd: ConnectNoC."]
        #[inline(always)]
        pub fn set_mctl_pwr_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Memory Controller NOC power controller is in IDLE state. 0x1: NOC logics in this power domain is in idle state; 0x0: no change."]
        #[inline(always)]
        pub const fn mctl_lpi_idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller NOC power controller is in IDLE state. 0x1: NOC logics in this power domain is in idle state; 0x0: no change."]
        #[inline(always)]
        pub fn set_mctl_lpi_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Memory Controller NOC power controller is in WORK state. 0x1: NOC logics in this power domain is in mission state; 0x0: no change."]
        #[inline(always)]
        pub const fn mctl_lpi_work(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller NOC power controller is in WORK state. 0x1: NOC logics in this power domain is in mission state; 0x0: no change."]
        #[inline(always)]
        pub fn set_mctl_lpi_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1: ddrc core work"]
        #[inline(always)]
        pub const fn mctl_ddrc_work(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc core work"]
        #[inline(always)]
        pub fn set_mctl_ddrc_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1: ddrc core idle"]
        #[inline(always)]
        pub const fn mctl_ddrc_idle(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc core idle"]
        #[inline(always)]
        pub fn set_mctl_ddrc_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1: ddrc port0 work"]
        #[inline(always)]
        pub const fn mctl_axi0_work(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port0 work"]
        #[inline(always)]
        pub fn set_mctl_axi0_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1: ddrc port0 idle"]
        #[inline(always)]
        pub const fn mctl_axi0_idle(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port0 idle"]
        #[inline(always)]
        pub fn set_mctl_axi0_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1: ddrc port1 work"]
        #[inline(always)]
        pub const fn mctl_axi1_work(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port1 work"]
        #[inline(always)]
        pub fn set_mctl_axi1_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1: ddrc port1 idle"]
        #[inline(always)]
        pub const fn mctl_axi1_idle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port1 idle"]
        #[inline(always)]
        pub fn set_mctl_axi1_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1: ddrc port2 work"]
        #[inline(always)]
        pub const fn mctl_axi2_work(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port2 work"]
        #[inline(always)]
        pub fn set_mctl_axi2_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1: ddrc port2 idle"]
        #[inline(always)]
        pub const fn mctl_axi2_idle(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port2 idle"]
        #[inline(always)]
        pub fn set_mctl_axi2_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1: ddrc port3 work"]
        #[inline(always)]
        pub const fn mctl_axi3_work(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port3 work"]
        #[inline(always)]
        pub fn set_mctl_axi3_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "1: ddrc port3 idle"]
        #[inline(always)]
        pub const fn mctl_axi3_idle(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port3 idle"]
        #[inline(always)]
        pub fn set_mctl_axi3_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "1: ddrc port4 work"]
        #[inline(always)]
        pub const fn mctl_axi4_work(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port4 work"]
        #[inline(always)]
        pub fn set_mctl_axi4_work(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "1: ddrc port4 idle"]
        #[inline(always)]
        pub const fn mctl_axi4_idle(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "1: ddrc port4 idle"]
        #[inline(always)]
        pub fn set_mctl_axi4_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "1: when power up, enter ddrc init state."]
        #[inline(always)]
        pub const fn ddrc_init(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "1: when power up, enter ddrc init state."]
        #[inline(always)]
        pub fn set_ddrc_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MctlLpiStat {
        #[inline(always)]
        fn default() -> MctlLpiStat {
            MctlLpiStat(0)
        }
    }
    #[doc = "Memory Controller Power LPI Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MctlPwrLpiCtl(pub u32);
    impl MctlPwrLpiCtl {
        #[doc = "Memory Controller power domain go POWER-OFF trigger. This bit will be cleared by a hardware ack signal automatically. 0x1: request Memory Controller power domain entering idle state; 0x0: no operation."]
        #[inline(always)]
        pub const fn mctl_pwr_off_req(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller power domain go POWER-OFF trigger. This bit will be cleared by a hardware ack signal automatically. 0x1: request Memory Controller power domain entering idle state; 0x0: no operation."]
        #[inline(always)]
        pub fn set_mctl_pwr_off_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Memory Controller power domain go POWER-ON trigger. This bit will be cleared by a hardware ack signal automatically. 0x1: request Memory Controller power domain entering PowerOn state; 0x0: no operation."]
        #[inline(always)]
        pub const fn mctl_pwr_up_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller power domain go POWER-ON trigger. This bit will be cleared by a hardware ack signal automatically. 0x1: request Memory Controller power domain entering PowerOn state; 0x0: no operation."]
        #[inline(always)]
        pub fn set_mctl_pwr_up_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory Controller NOC power controller go IDLE trigger. This is a wirte-only self-clearing bit. 0x1: request NOC logic in this power domain entering idle state; 0x0: no operation."]
        #[inline(always)]
        pub const fn mctl_lpi_go_idle(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller NOC power controller go IDLE trigger. This is a wirte-only self-clearing bit. 0x1: request NOC logic in this power domain entering idle state; 0x0: no operation."]
        #[inline(always)]
        pub fn set_mctl_lpi_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Memory Controller NOC power controller exit IDLE trigger. This is a wirte-only self-clearing bit. 0x1: request NOC logic in this power domain exiting idle state; 0x0: no operation."]
        #[inline(always)]
        pub const fn mctl_lpi_exit_idle(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Controller NOC power controller exit IDLE trigger. This is a wirte-only self-clearing bit. 0x1: request NOC logic in this power domain exiting idle state; 0x0: no operation."]
        #[inline(always)]
        pub fn set_mctl_lpi_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ddrc core low power request"]
        #[inline(always)]
        pub const fn mctl_go_idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc core low power request"]
        #[inline(always)]
        pub fn set_mctl_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ddrc axi port0 clock domain low power request"]
        #[inline(always)]
        pub const fn mctl_axi0_go_idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port0 clock domain low power request"]
        #[inline(always)]
        pub fn set_mctl_axi0_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ddrc axi port1 clock domain low power request"]
        #[inline(always)]
        pub const fn mctl_axi1_go_idle(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port1 clock domain low power request"]
        #[inline(always)]
        pub fn set_mctl_axi1_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ddrc axi port2 clock domain low power request"]
        #[inline(always)]
        pub const fn mctl_axi2_go_idle(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port2 clock domain low power request"]
        #[inline(always)]
        pub fn set_mctl_axi2_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ddrc axi port3 clock domain low power request"]
        #[inline(always)]
        pub const fn mctl_axi3_go_idle(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port3 clock domain low power request"]
        #[inline(always)]
        pub fn set_mctl_axi3_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ddrc axi port4 clock domain low power request"]
        #[inline(always)]
        pub const fn mctl_axi4_go_idle(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port4 clock domain low power request"]
        #[inline(always)]
        pub fn set_mctl_axi4_go_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ddrc core exit low power request"]
        #[inline(always)]
        pub const fn mctl_exit_idle(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc core exit low power request"]
        #[inline(always)]
        pub fn set_mctl_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ddrc axi port0 clock domain exit low power request"]
        #[inline(always)]
        pub const fn mctl_axi0_exit_idle(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port0 clock domain exit low power request"]
        #[inline(always)]
        pub fn set_mctl_axi0_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ddrc axi port1 clock domain exit low power request"]
        #[inline(always)]
        pub const fn mctl_axi1_exit_idle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port1 clock domain exit low power request"]
        #[inline(always)]
        pub fn set_mctl_axi1_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ddrc axi port2 clock domain exit low power request"]
        #[inline(always)]
        pub const fn mctl_axi2_exit_idle(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port2 clock domain exit low power request"]
        #[inline(always)]
        pub fn set_mctl_axi2_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ddrc axi port3 clock domain exit low power request"]
        #[inline(always)]
        pub const fn mctl_axi3_exit_idle(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port3 clock domain exit low power request"]
        #[inline(always)]
        pub fn set_mctl_axi3_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ddrc axi port4 clock domain exit low power request"]
        #[inline(always)]
        pub const fn mctl_axi4_exit_idle(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ddrc axi port4 clock domain exit low power request"]
        #[inline(always)]
        pub fn set_mctl_axi4_exit_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "rmu set to 1 during reset phase after power up,when need to sleep, soft set this bit to 0"]
        #[inline(always)]
        pub const fn mctl_pwr_ok_in(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "rmu set to 1 during reset phase after power up,when need to sleep, soft set this bit to 0"]
        #[inline(always)]
        pub fn set_mctl_pwr_ok_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "when power up, first initialize ddrc, after initialize dessert core reset"]
        #[inline(always)]
        pub const fn mctl_ddrc_init_done(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "when power up, first initialize ddrc, after initialize dessert core reset"]
        #[inline(always)]
        pub fn set_mctl_ddrc_init_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "after initialize ddr phy, disable ddrc low power interface"]
        #[inline(always)]
        pub const fn mctl_dfi_init_done(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "after initialize ddr phy, disable ddrc low power interface"]
        #[inline(always)]
        pub fn set_mctl_dfi_init_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "0: power off, 1: clock off"]
        #[inline(always)]
        pub const fn mctl_pwr_mode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "0: power off, 1: clock off"]
        #[inline(always)]
        pub fn set_mctl_pwr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "1: use ddrc core low power interface to control ddrc core clock During ddr low power process this bit should be set to 0, otherwise ddrc core clock will be gated. This doesn fulfill ddrc requirement. During ddrc bring up clock should be stable."]
        #[inline(always)]
        pub const fn mctl_ddrc_core_clken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "1: use ddrc core low power interface to control ddrc core clock During ddr low power process this bit should be set to 0, otherwise ddrc core clock will be gated. This doesn fulfill ddrc requirement. During ddrc bring up clock should be stable."]
        #[inline(always)]
        pub fn set_mctl_ddrc_core_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "1: hardware auto trigger self refresh through ddrc low power interface 0: software control ddrc self refresh"]
        #[inline(always)]
        pub const fn mctl_hw_pwr_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1: hardware auto trigger self refresh through ddrc low power interface 0: software control ddrc self refresh"]
        #[inline(always)]
        pub fn set_mctl_hw_pwr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for MctlPwrLpiCtl {
        #[inline(always)]
        fn default() -> MctlPwrLpiCtl {
            MctlPwrLpiCtl(0)
        }
    }
}
