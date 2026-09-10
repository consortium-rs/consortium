use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use consortium_ipc::{Chan, Doorbell, MaybeSend};
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

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        self.tx.notify_one();
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
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
/// subscribes (`recv_bytes` calls `wait()` right before registering its waker),
/// standing in for the remote publishing its slot *in the scan→subscribe
/// window*: the interrupt the kernel counts but whose wake the fan-out drops.
/// Only `recv_bytes`'s post-subscribe re-scan can then deliver the message.
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

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        // Lossy, like the hardware doorbells: no stored permit.
        self.notify.notify_waiters();
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        // `recv_bytes` calls `wait()` at the subscribe point, right before it
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

/// A doorbell whose lines are independent, and which counts what it rings.
///
/// [`HostDoorbell`] and [`RacyDoorbell`] each model a single line and ignore the [`Chan`]
/// entirely, which is enough while a transport only ever rings one thing. Acknowledgement
/// gives each side a second line, and the whole reason it is a *second* line rather than a
/// reuse of the data line is that one line carries one waiter — so a fake that collapses
/// them would hide exactly the bug the design avoids.
///
/// Both sides share one bus and address lines by id, the way two cores share one doorbell
/// peripheral. Ringing is latching (`notify_one`), like [`HostDoorbell`]: these tests are
/// about acknowledgement semantics, and the lossy fan-out is covered separately by
/// [`RacyDoorbell`].
#[derive(Clone, Debug)]
pub struct LinedDoorbell {
    lines: Arc<Vec<Line>>,
}

#[derive(Debug, Default)]
struct Line {
    notify: Notify,
    rings: std::sync::atomic::AtomicUsize,
}

impl LinedDoorbell {
    /// A bus of `lines` independent lines, shared by cloning.
    pub fn bus(lines: usize) -> Self {
        Self {
            lines: Arc::new((0..lines).map(|_| Line::default()).collect()),
        }
    }

    /// How many times line `ch` has been rung, by either side.
    pub fn rings(&self, ch: Chan) -> usize {
        self.lines[ch.id() as usize]
            .rings
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[pin_project::pin_project]
pub struct LinedWaitFut<'a> {
    #[pin]
    notified: tokio::sync::futures::Notified<'a>,
    ch: Chan,
}

impl<'a> Future for LinedWaitFut<'a> {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.notified.poll(cx) {
            Poll::Ready(()) => Poll::Ready(Ok(*this.ch)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Doorbell for LinedDoorbell {
    type Error = Infallible;

    fn ring(&self, ch: Chan) -> Result<(), Infallible> {
        let line = &self.lines[ch.id() as usize];
        line.rings.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        line.notify.notify_one();
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        LinedWaitFut {
            notified: self.lines[ch.id() as usize].notify.notified(),
            ch,
        }
    }

    fn pending(&self, _ch: Chan) -> bool {
        false
    }
}
