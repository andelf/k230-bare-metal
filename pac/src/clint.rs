#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RISC-V Core Local Interruptor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clint {
    ptr: *mut u8,
}
unsafe impl Send for Clint {}
unsafe impl Sync for Clint {}
impl Clint {
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
    pub const fn msip(self, n: usize) -> crate::common::Reg<regs::Msip, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Machine timer compare value low"]
    #[inline(always)]
    pub const fn mtimecmpl(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4000usize + n * 8usize) as _) }
    }
    #[doc = "Machine timer compare value high"]
    #[inline(always)]
    pub const fn mtimecmph(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4004usize + n * 8usize) as _) }
    }
    #[doc = "Machine timer value low"]
    #[inline(always)]
    pub const fn mtimel(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbff8usize) as _) }
    }
    #[doc = "Machine timer value high"]
    #[inline(always)]
    pub const fn mtimeh(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbffcusize) as _) }
    }
    #[doc = "Supervisor software interrupt pending"]
    #[inline(always)]
    pub const fn ssip(self, n: usize) -> crate::common::Reg<regs::Ssip, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc000usize + n * 4usize) as _) }
    }
    #[doc = "Supervisor timer compare value low"]
    #[inline(always)]
    pub const fn stimecmpl(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd000usize + n * 8usize) as _) }
    }
    #[doc = "Supervisor timer compare value high"]
    #[inline(always)]
    pub const fn stimecmph(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd004usize + n * 8usize) as _) }
    }
    #[doc = "Supervisor timer value low"]
    #[inline(always)]
    pub const fn stimel(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdff8usize) as _) }
    }
    #[doc = "Supervisor timer value high"]
    #[inline(always)]
    pub const fn stimeh(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Machine software interrupt pending"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msip(pub u32);
    impl Msip {
        #[doc = "Machine software interrupt pending"]
        #[inline(always)]
        pub const fn msip(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Machine software interrupt pending"]
        #[inline(always)]
        pub fn set_msip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Msip {
        #[inline(always)]
        fn default() -> Msip {
            Msip(0)
        }
    }
    #[doc = "Supervisor software interrupt pending"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssip(pub u32);
    impl Ssip {
        #[doc = "Supervisor software interrupt pending"]
        #[inline(always)]
        pub const fn ssip(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Supervisor software interrupt pending"]
        #[inline(always)]
        pub fn set_ssip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ssip {
        #[inline(always)]
        fn default() -> Ssip {
            Ssip(0)
        }
    }
}
