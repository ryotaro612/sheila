use std::{
    rc, result,
    sync::{self, mpsc, Mutex},
    thread,
    time::Duration,
};

pub(crate) struct ReceiveFuture<T>
where
    T: Send,
{
    receiver: mpsc::Receiver<T>,
}

impl<'a, T: Send + 'static> ReceiveFuture<T> {
    pub(crate) fn new(receiver: mpsc::Receiver<T>, interval: Duration) -> Self {
        ReceiveFuture { receiver }
    }
}

impl<'a, T: std::fmt::Debug> std::future::Future for ReceiveFuture<T>
where
    T: Send,
{
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // let receiver_clone = self.receiver.clone();
        // thread::spawn(move || {
        //     receiver_clone.recv();
        // });
        std::task::Poll::Pending
        // match self.receiver.recv_timeout(Duration::from_millis(0)) {
        //     Ok(v) => std::task::Poll::Ready(v),
        //     Err(_) => {
        //         let waker = ctx.waker().clone();
        //         let interval = self.interval;
        //         thread::spawn(move || {
        //             thread::sleep(interval);
        //             waker.wake_by_ref();
        //         });
        //         std::task::Poll::Pending
        //     }
        // }
    }
}

pub(crate) struct Temp<T> {
    called: bool,
    result: sync::Arc<Mutex<Option<result::Result<T, String>>>>,
    arc_receiver: sync::Arc<sync::Mutex<mpsc::Receiver<T>>>,
}

impl<T: Send> Temp<T> {
    pub(crate) fn new(arc_receiver: sync::Arc<sync::Mutex<mpsc::Receiver<T>>>) -> Self {
        let receiver = arc_receiver.clone();
        let result: sync::Arc<Mutex<Option<result::Result<T, String>>>> =
            sync::Arc::new(Mutex::new(None));
        thread::spawn(move || {});
        Temp {
            called: false,
            result: sync::Arc::new(Mutex::new(None)),
            arc_receiver,
        }
    }
}

impl<T: Send + 'static> std::future::Future for Temp<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let m = self.get_mut();
        if !m.called {
            m.called = true;
            let c = m.result.clone();
            let rev = m.arc_receiver.clone();

            let waker = ctx.waker().clone();
            thread::spawn(move || {
                let v = rev.lock().map_err(|e| e.to_string()).unwrap();
                match v.recv() {
                    Ok(a) => {
                        let f: Option<result::Result<T, String>> = Some(Ok(a));
                        if let Ok(mut locked) = c.lock() {
                            *locked = f;
                        }
                        // let mut x = c.get_mut().unwrap();
                        // *x = f;
                    }
                    Err(e) => {}
                }
                waker.wake_by_ref();
            });
        }

        let res = m.result.clone();

        let f = res.lock().unwrap();
        let d = f.as_ref();
        match d {
            Some(x) => {}
            None => {}
        }

        //self.as_ref().result;
        //self.result;
        //let b = self.get_mut();
        // let c = self.get_mut();
        // c.called = false;
        // c.result.lock().unwrap();

        //b.called = false;

        //self.result.lock().unwrap();
        std::task::Poll::Pending
    }
}
