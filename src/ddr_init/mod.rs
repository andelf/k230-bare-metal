pub const DDR_REG_BASE: u32 = 0x9800_0000;

pub mod cammv;

//#define CONFIG_MEM_BASE_ADDR    0x00
//#define CONFIG_MEM_TOTAL_SIZE   (128 * 1024 * 1024)

const CONFIG_MEM_BASE_ADDR: u64 = 0x0; // 需要定义具体地址
const CONFIG_SYS_CACHELINE_SIZE: u64 = 64; // 示例值，需要根据实际配置

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
pub fn mem_test(start_address: u64, length: u32) -> bool {
    let mut addr = start_address;
    let mut len = length;
    let mut i = 0;
    let mut data: u32;

    while len > 0 {
        data = unsafe { core::ptr::read_volatile(addr as *const u32) };
        unsafe { core::ptr::write_volatile(addr as *mut u32, !data) };
        if unsafe { core::ptr::read_volatile(addr as *const u32) } != !data {
            return false;
        }
        addr += 4;
        len -= 4;
        i += 1;
        if i % (100 * 1024 * 1024 / 4) == 0 {
            println!("{}MiB ", i / 1024 / 1024);
        }
    }

    true
}
