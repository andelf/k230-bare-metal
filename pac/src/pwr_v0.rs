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
    pub const fn mctl_pwr_lpi_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "MCTRLI clock switch"]
    #[inline(always)]
    pub const fn mctl_clock_switch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "MCTRLI power status"]
    #[inline(always)]
    pub const fn mctl_lpi_stat(self) -> crate::common::Reg<u32, crate::common::RW> {
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
