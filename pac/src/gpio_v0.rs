#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General Purpose Input/Output. group0 registers is 0x9140_B000, group1 registers is 0x9140_C000"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port A data register"]
    #[inline(always)]
    pub const fn swporta_dr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port A Data Direction Register"]
    #[inline(always)]
    pub const fn swporta_ddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Port A data source register"]
    #[inline(always)]
    pub const fn swporta_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port B data register"]
    #[inline(always)]
    pub const fn swportb_dr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port B Data Direction Register"]
    #[inline(always)]
    pub const fn swportb_ddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port B data source register"]
    #[inline(always)]
    pub const fn swportb_ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Interrupt enable register Note This register is available only if Port A is configured"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Interrupt mask register Note This register is available only if Port A is configured to generate"]
    #[inline(always)]
    pub const fn intmask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Interrupt level Note This register is available only if Port A is configured to generate interrupts"]
    #[inline(always)]
    pub const fn inttype_level(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Interrupt polarity Note This register is available only if Port A is configured to generate interrupts"]
    #[inline(always)]
    pub const fn int_polarity(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Interrupt status Note This register is available only if Port A is configured to generate interrupts"]
    #[inline(always)]
    pub const fn intstatus(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Raw interrupt status Note This register is available only if Port A is configured to generate"]
    #[inline(always)]
    pub const fn raw_intstatus(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Debounce enable Note This register is available only if Port A is configured to generate interrupts"]
    #[inline(always)]
    pub const fn debounce(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Port A clear interrupt register Note This register is available only if Port A is configured to"]
    #[inline(always)]
    pub const fn porta_eoi(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "External port A register"]
    #[inline(always)]
    pub const fn ext_porta(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Port B external port register"]
    #[inline(always)]
    pub const fn ext_portb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Synchronization level"]
    #[inline(always)]
    pub const fn ls_sync(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "GPIO ID code"]
    #[inline(always)]
    pub const fn id_code(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Interrupt Both Edge type Note This register is available only if PORT A is configured to generate"]
    #[inline(always)]
    pub const fn int_bothedge(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "GPIO Component Version"]
    #[inline(always)]
    pub const fn ver_id_code(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "GPIO Configuration Register 2 This register is a read-only register that is present when the configuration"]
    #[inline(always)]
    pub const fn config_reg2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "GPIO Configuration Register 1 This register is present when the configuration parameter GPIO_ADD_ENCODED_PARAMS"]
    #[inline(always)]
    pub const fn config_reg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
}
