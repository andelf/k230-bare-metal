use embassy_time::Timer;

use crate::board_gpio::GpioError;

pub const DEFAULT_CYCLES: u32 = 3;
pub const DEFAULT_DELAY_MS: u64 = 180;

pub fn run(cycles: u32, delay_ms: u64) -> Result<(), GpioError> {
    let cycles = cycles.max(1);
    let delay_ms = delay_ms.max(1);

    crate::embassy_time_driver_impl::init();
    let result = crate::embassy_runtime::block_on(pattern(cycles, delay_ms));
    crate::embassy_time_driver_impl::deactivate();
    result
}

async fn pattern(cycles: u32, delay_ms: u64) -> Result<(), GpioError> {
    for _ in 0..cycles {
        show(true, false, false, delay_ms).await?;
        show(false, true, false, delay_ms).await?;
        show(false, false, true, delay_ms).await?;
        show(true, true, true, delay_ms).await?;
        show(false, false, false, delay_ms).await?;
    }

    crate::board_gpio::set_rgb(false, false, false)
}

async fn show(red: bool, green: bool, blue: bool, delay_ms: u64) -> Result<(), GpioError> {
    crate::board_gpio::set_rgb(red, green, blue)?;
    Timer::after_millis(delay_ms).await;
    Ok(())
}
