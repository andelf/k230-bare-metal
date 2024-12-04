#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWM. kendryte,pwm"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWM configuration register"]
    #[inline(always)]
    pub const fn pwmcfg(self) -> crate::common::Reg<regs::Pwmcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWM counter count value register"]
    #[inline(always)]
    pub const fn pwmcount(self) -> crate::common::Reg<regs::Pwmcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWM counter relatively straight register"]
    #[inline(always)]
    pub const fn pwms(self) -> crate::common::Reg<regs::Pwms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWM comparator registers"]
    #[inline(always)]
    pub const fn pwmcmp(self, n: usize) -> crate::common::Reg<regs::Pwmcmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWM Configuration Register Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwmcfg(pub u32);
    impl Pwmcfg {
        #[doc = "4-bit field to scale PWM counter value before comparator input"]
        #[inline(always)]
        pub const fn scale(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "4-bit field to scale PWM counter value before comparator input"]
        #[inline(always)]
        pub fn set_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Controls clearing of pwmcmpXip interrupt pending bits"]
        #[inline(always)]
        pub const fn sticky(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Controls clearing of pwmcmpXip interrupt pending bits"]
        #[inline(always)]
        pub fn set_sticky(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Controls automatic counter reset when pwms matches pwmcmp0"]
        #[inline(always)]
        pub const fn zerocomp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Controls automatic counter reset when pwms matches pwmcmp0"]
        #[inline(always)]
        pub fn set_zerocomp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Controls glitch prevention in PWM waveforms when changing pwmcmpX values"]
        #[inline(always)]
        pub const fn deglitch(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Controls glitch prevention in PWM waveforms when changing pwmcmpX values"]
        #[inline(always)]
        pub fn set_deglitch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Controls continuous PWM counter operation. Counter increments continuously when set."]
        #[inline(always)]
        pub const fn enalways(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Controls continuous PWM counter operation. Counter increments continuously when set."]
        #[inline(always)]
        pub fn set_enalways(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Controls one-shot PWM counter operation. Counter increments once and stops when enabled."]
        #[inline(always)]
        pub const fn noneshot(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Controls one-shot PWM counter operation. Counter increments once and stops when enabled."]
        #[inline(always)]
        pub fn set_noneshot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Per-comparator pwmcmp0center bit to generate center-aligned symmetric duty-cycle"]
        #[inline(always)]
        pub const fn center(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Per-comparator pwmcmp0center bit to generate center-aligned symmetric duty-cycle"]
        #[inline(always)]
        pub fn set_center(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Comparator 0 gang mode. When set, comparator 0 fires and raises pwm0gpio signal."]
        #[inline(always)]
        pub const fn gang(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Comparator 0 gang mode. When set, comparator 0 fires and raises pwm0gpio signal."]
        #[inline(always)]
        pub fn set_gang(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 24usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "The interrupt pending bits pwmcmp0ip. Changed at the start of the next PWM cycle."]
        #[inline(always)]
        pub const fn ip(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 28usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The interrupt pending bits pwmcmp0ip. Changed at the start of the next PWM cycle."]
        #[inline(always)]
        pub fn set_ip(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 28usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pwmcfg {
        #[inline(always)]
        fn default() -> Pwmcfg {
            Pwmcfg(0)
        }
    }
    #[doc = "PWM Comparator Registers Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwmcmp(pub u32);
    impl Pwmcmp {
        #[doc = "PWM comparator value"]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "PWM comparator value"]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Pwmcmp {
        #[inline(always)]
        fn default() -> Pwmcmp {
            Pwmcmp(0)
        }
    }
    #[doc = "PWM Counter Count Value Register Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwmcount(pub u32);
    impl Pwmcount {
        #[doc = "PWM counter count value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "PWM counter count value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Pwmcount {
        #[inline(always)]
        fn default() -> Pwmcount {
            Pwmcount(0)
        }
    }
    #[doc = "PWM Counter Relatively Straight Register Fields"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwms(pub u32);
    impl Pwms {
        #[doc = "The value of pwms is memory-mapped and can be read as a single cmpwidth-bit value over the bus"]
        #[inline(always)]
        pub const fn pwms(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The value of pwms is memory-mapped and can be read as a single cmpwidth-bit value over the bus"]
        #[inline(always)]
        pub fn set_pwms(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pwms {
        #[inline(always)]
        fn default() -> Pwms {
            Pwms(0)
        }
    }
}
