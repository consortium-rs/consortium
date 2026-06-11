use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use consortium_ipc::{Chan, Doorbell};
use tokio::sync::Notify;

#[derive(Debug)]
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
