use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use consortium_ipc::{Chan, Doorbell};
use tokio::sync::Notify;

#[derive(Clone, Debug)]
pub struct HostDoorbell {
    tx: Arc<Notify>,
    rx: Arc<Notify>,
}

impl HostDoorbell {
    pub fn pair() -> (Self, Self) {
        let a = Arc::new(Notify::new());
        let b = Arc::new(Notify::new());
        (
            Self {
                tx: a.clone(),
                rx: b.clone(),
            },
            Self { tx: b, rx: a },
        )
    }
}

#[pin_project::pin_project]
pub struct HostWaitFut<'a> {
    #[pin]
    notified: tokio::sync::futures::Notified<'a>,
    ch: Chan,
}

impl<'a> Future for HostWaitFut<'a> {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.notified.poll(cx) {
            Poll::Ready(()) => Poll::Ready(Ok(*this.ch)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Doorbell for HostDoorbell {
    type Error = Infallible;
    type WaitFut<'a>
        = HostWaitFut<'a>
    where
        Self: 'a;

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        self.tx.notify_one();
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        HostWaitFut {
            notified: self.rx.notified(),
            ch,
        }
    }

    fn pending(&self, _ch: Chan) -> bool {
        false
    }
}

/// A doorbell that models real *hardware* fan-out for lost-wakeup testing.
///
/// Unlike [`HostDoorbell`], which rings with the latching `notify_one()`, this
/// rings with the **lossy** `notify_waiters()` — wakes only already-registered
/// waiters, stores no permit — exactly like the `hsem`/`ipcc`/`mu`/`gpio`
/// doorbells. It also runs a one-shot hook at the moment the receiver
/// subscribes (`poll_recv` calls `wait()` right before registering its waker),
/// standing in for the remote publishing its slot *in the scan→subscribe
/// window*: the interrupt the kernel counts but whose wake the fan-out drops.
/// Only `poll_recv`'s post-subscribe re-scan can then deliver the message.
/// One-shot subscribe-time hook, shared so [`RacyDoorbell`] stays `Clone`.
type SubscribeHook = Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>;

#[derive(Clone)]
pub struct RacyDoorbell {
    notify: Arc<Notify>,
    on_subscribe: SubscribeHook,
}

impl std::fmt::Debug for RacyDoorbell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The boxed hook is not `Debug`; `Error<D>` only needs *some* impl.
        f.debug_struct("RacyDoorbell").finish_non_exhaustive()
    }
}

impl RacyDoorbell {
    /// Build a doorbell whose next `wait()` runs `hook` exactly once, simulating
    /// a remote publish landing in the scan→subscribe window.
    pub fn new(hook: impl FnOnce() + Send + 'static) -> Self {
        Self {
            notify: Arc::new(Notify::new()),
            on_subscribe: Arc::new(Mutex::new(Some(Box::new(hook)))),
        }
    }
}

#[pin_project::pin_project]
pub struct RacyWaitFut<'a> {
    #[pin]
    notified: tokio::sync::futures::Notified<'a>,
    ch: Chan,
}

impl<'a> Future for RacyWaitFut<'a> {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.notified.poll(cx) {
            Poll::Ready(()) => Poll::Ready(Ok(*this.ch)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Doorbell for RacyDoorbell {
    type Error = Infallible;
    type WaitFut<'a>
        = RacyWaitFut<'a>
    where
        Self: 'a;

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        // Lossy, like the hardware doorbells: no stored permit.
        self.notify.notify_waiters();
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        // `poll_recv` calls `wait()` at the subscribe point, right before it
        // registers the waker. Fire the one-shot hook here so the "remote
        // publish" lands inside the scan→subscribe window.
        if let Some(hook) = self.on_subscribe.lock().expect("hook lock").take() {
            hook();
        }
        RacyWaitFut {
            notified: self.notify.notified(),
            ch,
        }
    }

    fn pending(&self, _ch: Chan) -> bool {
        false
    }
}
