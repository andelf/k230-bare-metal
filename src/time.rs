pub const TIMEBASE_HZ: u64 = crate::STC_CLK as u64;

pub type RtosTick = u64;

#[derive(Copy, Clone)]
pub struct Deadline {
    expires_at: RtosTick,
}

pub fn ticks() -> RtosTick {
    rdtime64()
}

pub fn rtos_ticks() -> RtosTick {
    ticks()
}

pub fn rtos_timebase_hz() -> RtosTick {
    TIMEBASE_HZ
}

pub fn rtos_tick_delta(hz: u32) -> RtosTick {
    TIMEBASE_HZ / u64::from(hz)
}

pub fn rtos_deadline_after_ticks(ticks: RtosTick) -> Deadline {
    Deadline {
        expires_at: rtos_ticks().wrapping_add(ticks.max(1)),
    }
}

pub fn rtos_deadline_expired(deadline: Deadline) -> bool {
    deadline.expired()
}

pub fn set_machine_timer_at(tick: RtosTick) {
    let low = tick as u32;
    let high = (tick >> 32) as u32;

    pac::CLINT.mtimecmph(0).write_value(u32::MAX);
    pac::CLINT.mtimecmp(0).write_value(low);
    pac::CLINT.mtimecmph(0).write_value(high);
}

pub fn set_machine_timer_after_ticks(ticks: RtosTick) -> RtosTick {
    let target = rtos_ticks().wrapping_add(ticks.max(1));
    set_machine_timer_at(target);
    target
}

pub fn set_machine_timer_after_ms(ms: u64) -> RtosTick {
    set_machine_timer_after_ticks(ms_to_ticks(ms))
}

pub fn disable_machine_timer() {
    pac::CLINT.mtimecmph(0).write_value(u32::MAX);
    pac::CLINT.mtimecmp(0).write_value(u32::MAX);
}

#[inline(always)]
pub fn rdtime64() -> RtosTick {
    let value: RtosTick;
    unsafe {
        core::arch::asm!("rdtime {}", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

pub fn ticks_to_us(ticks: RtosTick) -> u64 {
    ((ticks as u128 * 1_000_000u128) / TIMEBASE_HZ as u128) as u64
}

pub fn ticks_to_ms(ticks: RtosTick) -> u64 {
    ((ticks as u128 * 1_000u128) / TIMEBASE_HZ as u128) as u64
}

pub fn us_to_ticks(us: u64) -> u64 {
    let numerator = us as u128 * TIMEBASE_HZ as u128;
    let ticks = (numerator + 999_999u128) / 1_000_000u128;
    ticks.max(1) as u64
}

pub fn ms_to_ticks(ms: u64) -> u64 {
    let numerator = ms as u128 * TIMEBASE_HZ as u128;
    let ticks = (numerator + 999u128) / 1_000u128;
    ticks.max(1) as u64
}

pub fn deadline_after_us(us: u64) -> Deadline {
    rtos_deadline_after_ticks(us_to_ticks(us))
}

pub fn delay_us(us: u64) {
    let deadline = deadline_after_us(us);
    while !deadline.expired() {
        core::hint::spin_loop();
    }
}

impl Deadline {
    pub fn expired(self) -> bool {
        (ticks().wrapping_sub(self.expires_at) as i64) >= 0
    }
}
