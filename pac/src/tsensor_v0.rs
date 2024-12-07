#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Temperature sensor. 0x9110_7000"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsensor {
    ptr: *mut u8,
}
unsafe impl Send for Tsensor {}
unsafe impl Sync for Tsensor {}
impl Tsensor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TSensor IP Configuration Register"]
    #[inline(always)]
    pub const fn tsen_w(self) -> crate::common::Reg<regs::TsenW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Read retured value of TSensor IP Register"]
    #[inline(always)]
    pub const fn tsen_r(self) -> crate::common::Reg<regs::TsenR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Temperature Sensor Read Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsenR(pub u32);
    impl TsenR {
        #[doc = "TSensor data out"]
        #[inline(always)]
        pub const fn ts_dout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "TSensor data out"]
        #[inline(always)]
        pub fn set_ts_dout(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ts_dout is valid when high"]
        #[inline(always)]
        pub const fn ts_dout_valid(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ts_dout is valid when high"]
        #[inline(always)]
        pub fn set_ts_dout_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for TsenR {
        #[inline(always)]
        fn default() -> TsenR {
            TsenR(0)
        }
    }
    #[doc = "Temperature Sensor Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsenW(pub u32);
    impl TsenW {
        #[doc = "TSensor enable, active high"]
        #[inline(always)]
        pub const fn ts_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TSensor enable, active high"]
        #[inline(always)]
        pub fn set_ts_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TSensor Output mode selection. 0:Single output mode 1:Continuous output mode"]
        #[inline(always)]
        pub const fn ts_conv_mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TSensor Output mode selection. 0:Single output mode 1:Continuous output mode"]
        #[inline(always)]
        pub fn set_ts_conv_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TSensor Reference Voltage calibration"]
        #[inline(always)]
        pub const fn ts_trim(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "TSensor Reference Voltage calibration"]
        #[inline(always)]
        pub fn set_ts_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Internal Reference Voltage test enable, active high"]
        #[inline(always)]
        pub const fn ts_test_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Internal Reference Voltage test enable, active high"]
        #[inline(always)]
        pub fn set_ts_test_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for TsenW {
        #[inline(always)]
        fn default() -> TsenW {
            TsenW(0)
        }
    }
}
