#![no_std]

use lazy_static::lazy_static;
use spin::Mutex;

use task::executor::Executor;

pub mod allocator;
pub mod console;
pub mod interrupts;
pub mod memory;
pub mod task;

/// K230 Kernel structure
pub struct K230Kernel {}

impl K230Kernel {
    pub fn new_executor(&self) -> Executor {
        Executor {}
    }

    pub fn init(&self) {}
}

lazy_static! {
    pub static ref KERNEL: Mutex<K230Kernel> = Mutex::new(K230Kernel {});
}

pub fn init() {
    KERNEL.lock().init()
}
