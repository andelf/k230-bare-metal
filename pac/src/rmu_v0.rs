#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset Management Unit. 0x9110_1000"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmu {
    ptr: *mut u8,
}
unsafe impl Send for Rmu {}
unsafe impl Sync for Rmu {}
impl Rmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "cpu0 reset time control"]
    #[inline(always)]
    pub const fn cpu0_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "cpu0 reset control"]
    #[inline(always)]
    pub const fn cpu0_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "cpu1 reset time control"]
    #[inline(always)]
    pub const fn cpu1_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "cpu1 reset control"]
    #[inline(always)]
    pub const fn cpu1_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "AI reset time control"]
    #[inline(always)]
    pub const fn ai_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "AI reset control"]
    #[inline(always)]
    pub const fn ai_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "VPU reset time control"]
    #[inline(always)]
    pub const fn vpu_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "VPU reset control"]
    #[inline(always)]
    pub const fn vpu_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "soc ctl reset control"]
    #[inline(always)]
    pub const fn soc_ctl_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "lo sys reset control"]
    #[inline(always)]
    pub const fn losys_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "hi sys reset time control"]
    #[inline(always)]
    pub const fn hisys_hclk_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "hi sys reset control"]
    #[inline(always)]
    pub const fn hisys_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "sd reset time control"]
    #[inline(always)]
    pub const fn sdctl_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "sd reset control"]
    #[inline(always)]
    pub const fn sdc_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "usb reset time control"]
    #[inline(always)]
    pub const fn usb_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "usb reset control"]
    #[inline(always)]
    pub const fn usb_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "spi reset time control"]
    #[inline(always)]
    pub const fn spi_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "spi reset control"]
    #[inline(always)]
    pub const fn spi_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "sec reset time control"]
    #[inline(always)]
    pub const fn sec_sys_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "sec reset control"]
    #[inline(always)]
    pub const fn sec_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "dmac reset time control"]
    #[inline(always)]
    pub const fn dmac_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "dmac reset control"]
    #[inline(always)]
    pub const fn dma_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "decompress reset time control"]
    #[inline(always)]
    pub const fn decompress_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "decompress reset control"]
    #[inline(always)]
    pub const fn decompress_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "sram reset time control"]
    #[inline(always)]
    pub const fn sram_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "sram reset control"]
    #[inline(always)]
    pub const fn sram_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "nonai-2d reset time control"]
    #[inline(always)]
    pub const fn nonai2d_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "nonai-2d reset control"]
    #[inline(always)]
    pub const fn nonai2d_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "ddrc reset time control"]
    #[inline(always)]
    pub const fn mctl_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "ddrc reset control"]
    #[inline(always)]
    pub const fn mctl_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "isp reset time control"]
    #[inline(always)]
    pub const fn isp_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "isp de reset time control"]
    #[inline(always)]
    pub const fn isp_dw_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "isp reset control"]
    #[inline(always)]
    pub const fn isp_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "DPU reset time control"]
    #[inline(always)]
    pub const fn dpu_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "DPU reset control"]
    #[inline(always)]
    pub const fn dpu_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "display reset time control"]
    #[inline(always)]
    pub const fn disp_sys_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "display reset control"]
    #[inline(always)]
    pub const fn disp_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "video 2.5D reset time control"]
    #[inline(always)]
    pub const fn v2p5d_sys_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "video 2.5D reset control"]
    #[inline(always)]
    pub const fn v2p5d_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "audio reset time control"]
    #[inline(always)]
    pub const fn audio_rst_tim(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "audio reset control"]
    #[inline(always)]
    pub const fn audio_rst_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
}
