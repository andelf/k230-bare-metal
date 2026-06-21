use core::{
    arch::asm,
    future::Future,
    pin::pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut cx = Context::from_waker(&waker);
    let mut future = pin!(future);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => unsafe {
                asm!("wfi");
            },
        }
    }
}

fn raw_waker() -> RawWaker {
    RawWaker::new(core::ptr::null(), &WAKER_VTABLE)
}

unsafe fn clone(_: *const ()) -> RawWaker {
    raw_waker()
}

unsafe fn wake(_: *const ()) {}

unsafe fn wake_by_ref(_: *const ()) {}

unsafe fn drop(_: *const ()) {}

static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
