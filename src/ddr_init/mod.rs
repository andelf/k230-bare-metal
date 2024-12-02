pub const DDR_REG_BASE: u32 = 0x9800_0000;

pub mod cammv;

//#define CONFIG_MEM_BASE_ADDR    0x00
//#define CONFIG_MEM_TOTAL_SIZE   (128 * 1024 * 1024)

pub const CONFIG_MEM_BASE_ADDR: u64 = 0x0; // 需要定义具体地址

pub use cammv::board_ddr_init;

pub fn ddr_init_training() {
    const ADDR: u32 = 0x980001bc;

    unsafe {
        if core::ptr::read_volatile(ADDR as *const u32) & 1 == 0 {
            board_ddr_init();
        }
    }
}

// Run memory test using u32 word, print status every 100MiB
pub fn mem_test(start_address: u64, length: u32, pattern: u32) -> bool {

