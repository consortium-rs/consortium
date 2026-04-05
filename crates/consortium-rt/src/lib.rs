#![no_std]

use core::future::Future;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// --- Noop waker -----------------------------------------------------------
// A waker whose only purpose is to satisfy the `Context` requirement of
// `Future::poll`.  When woken, it does nothing; the executor re-polls
// immediately in the next loop iteration (spin executor).

unsafe fn noop_clone(_ptr: *const ()) -> RawWaker {
    noop_raw_waker()
}
unsafe fn noop_wake(_ptr: *const ()) {}
unsafe fn noop_wake_by_ref(_ptr: *const ()) {}
unsafe fn noop_drop(_ptr: *const ()) {}

static NOOP_VTABLE: RawWakerVTable =
    RawWakerVTable::new(noop_clone, noop_wake, noop_wake_by_ref, noop_drop);

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(core::ptr::null(), &NOOP_VTABLE)
}

/// Returns a [`Waker`] that does nothing when woken.
///
/// Suitable for use with spin-polling executors where the caller polls
/// manually rather than relying on asynchronous notifications.
pub fn noop_waker() -> Waker {
    // SAFETY: the vtable functions are all no-ops and the data pointer is
    // never dereferenced.
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

// --- Spin executor --------------------------------------------------------

/// Block the current thread (or bare-metal core) until `future` resolves.
///
/// This is a cooperative spin executor suitable for `no_std` environments
/// such as Cortex-M targets where no OS scheduler is present.  The future
/// is polled in a tight loop; when it returns [`Poll::Pending`] the loop
/// immediately polls again without yielding to any other task.
///
/// For platforms with sleep/WFI instructions, prefer using interrupt-driven
/// wakers rather than this function so that the core can enter low-power
/// states between wakeups.
///
/// # Example
///
/// ```rust,ignore
/// // On a Cortex-M target (no_std):
/// let result = consortium_rt::block_on(async {
///     some_async_operation().await
/// });
/// ```
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut pinned = core::pin::pin!(future);

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::pin::Pin;

    #[test]
    fn block_on_ready_immediately() {
        let result = block_on(async { 42u32 });
        assert_eq!(result, 42);
    }

    #[test]
    fn block_on_pending_then_ready() {
        // A future that returns Pending on the first poll and Ready on the second.
        struct OncePending {
            polled: bool,
        }

        impl Future for OncePending {
            type Output = u32;

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
                if self.polled {
                    Poll::Ready(7)
                } else {
                    self.polled = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }

        let result = block_on(OncePending { polled: false });
        assert_eq!(result, 7);
    }

    #[test]
    fn noop_waker_does_not_panic() {
        let waker = noop_waker();
        waker.wake_by_ref();
        let _cloned = waker.clone();
    }
}
