#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Clock Management Unit. 0x9110_0000"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmu {
    ptr: *mut u8,
}
unsafe impl Send for Cmu {}
unsafe impl Sync for Cmu {}
impl Cmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "cpu0 clk configuration"]
    #[inline(always)]
    pub const fn cpu0_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "cpu1 clk cofiguration"]
    #[inline(always)]
    pub const fn cpu1_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ai clk cofiguration"]
    #[inline(always)]
    pub const fn ai_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "vpu clk cofiguration"]
    #[inline(always)]
    pub const fn vpu_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "pmu clk cofiguration"]
    #[inline(always)]
    pub const fn pmu_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "high speed system clk cofiguration"]
    #[inline(always)]
    pub const fn hs_clken_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SD clk configuration"]
    #[inline(always)]
    pub const fn hs_sdclk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SPI clk configuration"]
    #[inline(always)]
    pub const fn hs_spi_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "low speed system clk configuration"]
    #[inline(always)]
    pub const fn ls_clken_cfg0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "low speed system clk configuration"]
    #[inline(always)]
    pub const fn ls_clken_cfg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "low speed uart, iic clock divider configuration"]
    #[inline(always)]
    pub const fn uart_i2c_clkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "low speed clk divider configuration"]
    #[inline(always)]
    pub const fn ls_clkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "audio clk divider configuration"]
    #[inline(always)]
    pub const fn audio_clkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "audio codec adc clk divider configuration"]
    #[inline(always)]
    pub const fn codec_adc_mclkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "audio codec dac clk divider configuration"]
    #[inline(always)]
    pub const fn codec_dac_mclkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "pdm clk divider configuration"]
    #[inline(always)]
    pub const fn pdm_clkdiv_cfg0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "pdm clk divider configuration"]
    #[inline(always)]
    pub const fn pdm_clkdiv_cfg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "sys ctrl clk configuration"]
    #[inline(always)]
    pub const fn sysctl_clken_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "timer clk configuration"]
    #[inline(always)]
    pub const fn timer_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "sys ctl clk divider configuration"]
    #[inline(always)]
    pub const fn sysctl_clk_div_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "share memory system clk configuration"]
    #[inline(always)]
    pub const fn shrm_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "DDR system clk configuration"]
    #[inline(always)]
    pub const fn ddr_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "ISP clk configuration"]
    #[inline(always)]
    pub const fn isp_clken_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "ISP clk divider configuration"]
    #[inline(always)]
    pub const fn isp_clkdiv_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "MCLK clk configuration"]
    #[inline(always)]
    pub const fn mclk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "DPU clk configuration"]
    #[inline(always)]
    pub const fn dpu_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "display clk configuration"]
    #[inline(always)]
    pub const fn vo_clk_cfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "display clk divider configuration"]
    #[inline(always)]
    pub const fn disp_clk_div(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "security clk divider configuration"]
    #[inline(always)]
    pub const fn sec_clk_div(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "usb test clk divider configuration"]
    #[inline(always)]
    pub const fn usb_test_clk_div(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "MIPI dphy test clk divider configuration"]
    #[inline(always)]
    pub const fn dphy_test_clk_div(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
