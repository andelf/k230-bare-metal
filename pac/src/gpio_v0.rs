#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DW_apb_gpio, General Purpose I/O"]
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
    #[doc = "SWPORT abstraction."]
    #[inline(always)]
    pub const fn swport(self, n: usize) -> Port {
        assert!(n < 4usize);
        unsafe { Port::from_ptr(self.ptr.add(0x0usize + n * 12usize) as _) }
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
    #[doc = "Interrupt level Note This register is available only if Port A is configured to generate interrupts. 0 = level, 1 = edge"]
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
    #[doc = "External port register"]
    #[inline(always)]
    pub const fn ext_port(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
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
    pub const fn config_reg2(self) -> crate::common::Reg<regs::ConfigReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "GPIO Configuration Register 1 This register is present when the configuration parameter GPIO_ADD_ENCODED_PARAMS"]
    #[inline(always)]
    pub const fn config_reg1(self) -> crate::common::Reg<regs::ConfigReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
}
#[doc = "SWPORT abstraction."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port {
    ptr: *mut u8,
}
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port Data Direction Register. 0 = input, 1 = output"]
    #[inline(always)]
    pub const fn ddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Port data source register. 0 = software, 1 = hardware"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "GPIO Configuration Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConfigReg1(pub u32);
    impl ConfigReg1 {
        #[doc = "GPIO_APB_DATA_WIDTH"]
        #[inline(always)]
        pub const fn apb_data_width(&self) -> super::vals::ApbDataWidth {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::ApbDataWidth::from_bits(val as u8)
        }
        #[doc = "GPIO_APB_DATA_WIDTH"]
        #[inline(always)]
        pub fn set_apb_data_width(&mut self, val: super::vals::ApbDataWidth) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "GPIO_NUM_PORT - 1"]
        #[inline(always)]
        pub const fn num_ports(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "GPIO_NUM_PORT - 1"]
        #[inline(always)]
        pub fn set_num_ports(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "GPIO_PORTx_SINGLE_CTL, if port is controlled from a single source"]
        #[inline(always)]
        pub const fn port_single_ctl(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO_PORTx_SINGLE_CTL, if port is controlled from a single source"]
        #[inline(always)]
        pub fn set_port_single_ctl(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "GPIO_HW_PORTx, if port has external, auxiliary hardware signals"]
        #[inline(always)]
        pub const fn hw_port(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO_HW_PORTx, if port has external, auxiliary hardware signals"]
        #[inline(always)]
        pub fn set_hw_port(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "GPIO_PORTA_INTR, if PORT is used as an interrupt source"]
        #[inline(always)]
        pub const fn porta_intr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO_PORTA_INTR, if PORT is used as an interrupt source"]
        #[inline(always)]
        pub fn set_porta_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPIO_DEBOUNCE, if include debounce capability"]
        #[inline(always)]
        pub const fn debounce(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO_DEBOUNCE, if include debounce capability"]
        #[inline(always)]
        pub fn set_debounce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIO_ADD_ENCODED_PARAMS, if encoded parameters added"]
        #[inline(always)]
        pub const fn add_encoded_params(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO_ADD_ENCODED_PARAMS, if encoded parameters added"]
        #[inline(always)]
        pub fn set_add_encoded_params(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIO_ID included"]
        #[inline(always)]
        pub const fn gpio_id(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO_ID included"]
        #[inline(always)]
        pub fn set_gpio_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "GPIO_ID_WIDTH"]
        #[inline(always)]
        pub const fn encoded_id_width(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "GPIO_ID_WIDTH"]
        #[inline(always)]
        pub fn set_encoded_id_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "GPIO_INT_BOTH_EDGE, if interrupt generation on both rising and falling edge"]
        #[inline(always)]
        pub const fn interrupt_both_edge_type(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO_INT_BOTH_EDGE, if interrupt generation on both rising and falling edge"]
        #[inline(always)]
        pub fn set_interrupt_both_edge_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for ConfigReg1 {
        #[inline(always)]
        fn default() -> ConfigReg1 {
            ConfigReg1(0)
        }
    }
    #[doc = "GPIO Configuration Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConfigReg2(pub u32);
    impl ConfigReg2 {
        #[doc = "GPIO_PWIDTH_x-1"]
        #[inline(always)]
        pub const fn encoded_id_pwidth(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "GPIO_PWIDTH_x-1"]
        #[inline(always)]
        pub fn set_encoded_id_pwidth(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for ConfigReg2 {
        #[inline(always)]
        fn default() -> ConfigReg2 {
            ConfigReg2(0)
        }
    }
}
pub mod vals {
    #[doc = "APB data width"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ApbDataWidth {
        _8BITS = 0x0,
        _16BITS = 0x01,
        _32BITS = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl ApbDataWidth {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ApbDataWidth {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ApbDataWidth {
        #[inline(always)]
        fn from(val: u8) -> ApbDataWidth {
            ApbDataWidth::from_bits(val)
        }
    }
    impl From<ApbDataWidth> for u8 {
        #[inline(always)]
        fn from(val: ApbDataWidth) -> u8 {
            ApbDataWidth::to_bits(val)
        }
    }
}
