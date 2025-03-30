use std::{
    fmt::Debug,
    result,
    sync::{
        self,
        mpsc::{self, RecvError},
        Mutex,
    },
    thread,
};

pub(crate) struct ReceivedFuture<T> {
    called: bool,
    receiver: sync::Arc<sync::Mutex<mpsc::Receiver<T>>>,
    result: sync::Arc<Mutex<Option<result::Result<T, RecvError>>>>,
}

impl<T: Send + Debug + 'static> ReceivedFuture<T> {
    pub(crate) fn new(receiver: sync::Arc<sync::Mutex<mpsc::Receiver<T>>>) -> Self {
        ReceivedFuture {
            called: false,
            receiver,
            result: sync::Arc::new(Mutex::new(None)),
        }
    }
}
impl<T: Clone + Debug + Send + 'static> std::future::Future for ReceivedFuture<T> {
    type Output = result::Result<T, RecvError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        if !this.called {
            this.called = true;
            let rev = this.receiver.clone();
            let res = this.result.clone();
            let waker = ctx.waker().clone();
            thread::spawn(move || {
                let received = rev.lock().unwrap().recv();
                log::debug!("received: {:?}", received);
                res.lock().unwrap().replace(received);
                waker.wake_by_ref();
            });
        }
        match this.result.lock().unwrap().as_ref() {
            Some(Ok(v)) => {
                return std::task::Poll::Ready(Ok(v.clone()));
            }
            Some(Err(e)) => {
                return std::task::Poll::Ready(Err(e.clone()));
            }
            None => std::task::Poll::Pending,
        }
    }
}
