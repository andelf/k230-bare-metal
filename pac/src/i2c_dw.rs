#![allow(clippy::missing_safety_doc)]

#[doc = "DW_apb_i2c register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}

impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }

    #[inline(always)]
    pub const fn con(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x00) as _) }
    }

    #[inline(always)]
    pub const fn tar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04) as _) }
    }

    #[inline(always)]
    pub const fn data_cmd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10) as _) }
    }

    #[inline(always)]
    pub const fn ss_scl_hcnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14) as _) }
    }

    #[inline(always)]
    pub const fn ss_scl_lcnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18) as _) }
    }

    #[inline(always)]
    pub const fn fs_scl_hcnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c) as _) }
    }

    #[inline(always)]
    pub const fn fs_scl_lcnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20) as _) }
    }

    #[inline(always)]
    pub const fn intr_stat(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2c) as _) }
    }

    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30) as _) }
    }

    #[inline(always)]
    pub const fn raw_intr_stat(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34) as _) }
    }

    #[inline(always)]
    pub const fn rx_tl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38) as _) }
    }

    #[inline(always)]
    pub const fn tx_tl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3c) as _) }
    }

    #[inline(always)]
    pub const fn clr_intr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40) as _) }
    }

    #[inline(always)]
    pub const fn clr_tx_abrt(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54) as _) }
    }

    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6c) as _) }
    }

    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70) as _) }
    }

    #[inline(always)]
    pub const fn txflr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74) as _) }
    }

    #[inline(always)]
    pub const fn rxflr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78) as _) }
    }

    #[inline(always)]
    pub const fn sda_hold(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7c) as _) }
    }

    #[inline(always)]
    pub const fn tx_abrt_source(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80) as _) }
    }

    #[inline(always)]
    pub const fn enable_status(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9c) as _) }
    }

    #[inline(always)]
    pub const fn comp_param_1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4) as _) }
    }

    #[inline(always)]
    pub const fn comp_version(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8) as _) }
    }

    #[inline(always)]
    pub const fn comp_type(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfc) as _) }
    }
}
