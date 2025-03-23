use std::{sync::mpsc, thread, time::Duration};
struct ReceiveFuture<'a, T> {
    receiver: &'a mpsc::Receiver<T>,
    interval: Duration,
}

impl<'a, T: std::fmt::Debug> std::future::Future for ReceiveFuture<'a, T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.receiver.recv_timeout(Duration::from_millis(0)) {
            Ok(v) => std::task::Poll::Ready(v),
            Err(_) => {
                let waker = ctx.waker().clone();
                let interval = self.interval;
                thread::spawn(move || {
                    thread::sleep(interval);
                    waker.wake_by_ref();
                });
                std::task::Poll::Pending
            }
        }
    }
}
