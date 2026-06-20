use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{RawWaker, RawWakerVTable, Waker};

use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;

const HARDWARE_HZ: u64 = crate::STC_CLK as u64;
static DRIVER_ACTIVE: AtomicBool = AtomicBool::new(false);
static PROBE_WOKEN: AtomicBool = AtomicBool::new(false);

pub struct MachineTimerDriver {
    queue: Mutex<CriticalSectionRawMutex, UnsafeCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: MachineTimerDriver = MachineTimerDriver {
    queue: Mutex::const_new(CriticalSectionRawMutex::new(), UnsafeCell::new(Queue::new())),
});

impl MachineTimerDriver {
    fn reset(&self) {
        critical_section::with(|cs| unsafe {
            core::ptr::write(self.queue.borrow(cs).get(), Queue::new());
        });
    }

    fn next_expiration(&self, cs: CriticalSection) -> u64 {
        unsafe { &mut *self.queue.borrow(cs).get() }.next_expiration(self.now())
    }

    fn arm_next_expiration(&self, cs: CriticalSection) {
        let mut next = self.next_expiration(cs);
        while !self.set_alarm(cs, next) {
            next = self.next_expiration(cs);
        }
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let _ = cs;

        if timestamp == u64::MAX {
            crate::time::disable_machine_timer();
            unsafe {
                riscv::register::mie::clear_mtimer();
            }
            return true;
        }

        let now = self.now();
        if timestamp <= now {
            crate::time::disable_machine_timer();
            return false;
        }

        let hardware_tick = embassy_tick_to_hardware_tick(timestamp.saturating_add(1));
        crate::time::set_machine_timer_at(hardware_tick);

        unsafe {
            riscv::register::mie::set_mtimer();
        }

        self.now() < timestamp
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        self.arm_next_expiration(cs);
    }

    fn on_interrupt(&self) {
        unsafe {
            riscv::register::mie::clear_mtimer();
        }
        critical_section::with(|cs| self.trigger_alarm(cs));
    }
}

impl Driver for MachineTimerDriver {
    fn now(&self) -> u64 {
        hardware_tick_to_embassy_tick(crate::time::rdtime64())
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let should_update =
                unsafe { &mut *self.queue.borrow(cs).get() }.schedule_wake(at, waker);
            if should_update {
                self.arm_next_expiration(cs);
            }
        });
    }
}

pub fn init() {
    crate::time::disable_machine_timer();
    DRIVER_ACTIVE.store(false, Ordering::Release);
    PROBE_WOKEN.store(false, Ordering::Release);
    DRIVER.reset();
    DRIVER_ACTIVE.store(true, Ordering::Release);
    unsafe {
        riscv::register::mstatus::set_mie();
    }
}

pub fn deactivate() {
    DRIVER_ACTIVE.store(false, Ordering::Release);
}

pub fn on_machine_timer_interrupt() -> bool {
    if !DRIVER_ACTIVE.load(Ordering::Acquire) {
        crate::time::disable_machine_timer();
        unsafe {
            riscv::register::mie::clear_mtimer();
        }
        return false;
    }

    DRIVER.on_interrupt();
    true
}

pub fn probe_wake_after_ms(ms: u64) -> bool {
    PROBE_WOKEN.store(false, Ordering::Release);

    let now = Driver::now(&DRIVER);
    let delay_ticks = ms.saturating_mul(TICK_HZ / 1_000);
    let wake_at = now.saturating_add(delay_ticks.max(1));
    let timeout_at = wake_at.saturating_add(TICK_HZ / 10);
    let waker = unsafe { Waker::from_raw(probe_raw_waker()) };

    Driver::schedule_wake(&DRIVER, wake_at, &waker);

    while Driver::now(&DRIVER) < timeout_at {
        if PROBE_WOKEN.load(Ordering::Acquire) {
            return true;
        }
        core::hint::spin_loop();
    }

    PROBE_WOKEN.load(Ordering::Acquire)
}

#[inline(always)]
fn hardware_tick_to_embassy_tick(tick: u64) -> u64 {
    ((tick as u128 * TICK_HZ as u128) / HARDWARE_HZ as u128) as u64
}

#[inline(always)]
fn embassy_tick_to_hardware_tick(tick: u64) -> u64 {
    ((tick as u128 * HARDWARE_HZ as u128) / TICK_HZ as u128) as u64
}

fn probe_raw_waker() -> RawWaker {
    RawWaker::new(core::ptr::null(), &PROBE_WAKER_VTABLE)
}

unsafe fn probe_clone(_: *const ()) -> RawWaker {
    probe_raw_waker()
}

unsafe fn probe_wake(_: *const ()) {
    PROBE_WOKEN.store(true, Ordering::Release);
}

unsafe fn probe_wake_by_ref(_: *const ()) {
    PROBE_WOKEN.store(true, Ordering::Release);
}

unsafe fn probe_drop(_: *const ()) {}

static PROBE_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(probe_clone, probe_wake, probe_wake_by_ref, probe_drop);
