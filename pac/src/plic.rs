#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hart block - threshold and claim"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HartCtrl {
    ptr: *mut u8,
}
unsafe impl Send for HartCtrl {}
unsafe impl Sync for HartCtrl {}
impl HartCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "machine mode threshold register"]
    #[inline(always)]
    pub const fn mth(self) -> crate::common::Reg<regs::Threshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "machine mode claim and complete register"]
    #[inline(always)]
    pub const fn mclaim(self) -> crate::common::Reg<regs::Claim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "supervisor mode threshold register"]
    #[inline(always)]
    pub const fn sth(self) -> crate::common::Reg<regs::Threshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "supervisor mode claim and complete register"]
    #[inline(always)]
    pub const fn sclaim(self) -> crate::common::Reg<regs::Claim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize) as _) }
    }
}
#[doc = "Hart block - MIE and SIE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HartIe {
    ptr: *mut u8,
}
unsafe impl Send for HartIe {}
unsafe impl Sync for HartIe {}
impl HartIe {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Machine software interrupt pending"]
    #[inline(always)]
    pub const fn mie(self, n: usize) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Supervisor software interrupt pending"]
    #[inline(always)]
    pub const fn sie(self, n: usize) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
#[doc = "PLIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plic {
    ptr: *mut u8,
}
unsafe impl Send for Plic {}
unsafe impl Sync for Plic {}
impl Plic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt source priority."]
    #[inline(always)]
    pub const fn prio(self, n: usize) -> crate::common::Reg<regs::Prio, crate::common::RW> {
        assert!(n < 1024usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt wait register."]
    #[inline(always)]
    pub const fn ip(self, n: usize) -> crate::common::Reg<regs::Ip, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn hart_ie(self, n: usize) -> HartIe {
        assert!(n < 256usize);
        unsafe { HartIe::from_ptr(self.ptr.add(0x2000usize + n * 256usize) as _) }
    }
    #[doc = "PLIC control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x001f_fffcusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn hart_ctrl(self, n: usize) -> HartCtrl {
        assert!(n < 256usize);
        unsafe { HartCtrl::from_ptr(self.ptr.add(0x0020_0000usize + n * 8192usize) as _) }
    }
}
pub mod regs {
    #[doc = "Claim and complete register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Claim(pub u32);
    impl Claim {
        #[doc = "Claim register."]
        #[inline(always)]
        pub const fn claim_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Claim register."]
        #[inline(always)]
        pub fn set_claim_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Claim {
        #[inline(always)]
        fn default() -> Claim {
            Claim(0)
        }
    }
    #[doc = "PLIC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Permission bit for supervisor mode."]
        #[inline(always)]
        pub const fn s_per(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Permission bit for supervisor mode."]
        #[inline(always)]
        pub fn set_s_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Enable(pub u32);
    impl Enable {
        #[doc = "Enable bit for each interrupt source."]
        #[inline(always)]
        pub const fn enable(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable bit for each interrupt source."]
        #[inline(always)]
        pub fn set_enable(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Enable {
        #[inline(always)]
        fn default() -> Enable {
            Enable(0)
        }
    }
    #[doc = "Pending interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ip(pub u32);
    impl Ip {
        #[doc = "Interrupt pending status of interrupt sources."]
        #[inline(always)]
        pub const fn ip(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of interrupt sources."]
        #[inline(always)]
        pub fn set_ip(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ip {
        #[inline(always)]
        fn default() -> Ip {
            Ip(0)
        }
    }
    #[doc = "Priority of each interrupt source."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prio(pub u32);
    impl Prio {
        #[doc = "Priority of each interrupt source."]
        #[inline(always)]
        pub const fn prio(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Priority of each interrupt source."]
        #[inline(always)]
        pub fn set_prio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Prio {
        #[inline(always)]
        fn default() -> Prio {
            Prio(0)
        }
    }
    #[doc = "Priority threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Threshold(pub u32);
    impl Threshold {
        #[doc = "Priority threshold."]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Priority threshold."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Threshold {
        #[inline(always)]
        fn default() -> Threshold {
            Threshold(0)
        }
    }
}
