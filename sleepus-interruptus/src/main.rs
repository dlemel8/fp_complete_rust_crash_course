use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use async_std::task::{Context, Poll, sleep, spawn};
use pin_project_lite::pin_project;

struct SleepPrint<Fut> {
    sleep: Fut,
}

impl<Fut: Future<Output=()>> Future for SleepPrint<Fut> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let sleep: Pin<&mut Fut> =
            unsafe { self.map_unchecked_mut(|s| &mut s.sleep) };

        match sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                println!("Inside SleepPrint");
                Poll::Ready(())
            }
        }
    }
}

pin_project! {
    struct TwoFutures<Fut1, Fut2>{
        first_done: bool,
        #[pin]
        first: Fut1,
        #[pin]
        second: Fut2,
    }
}

impl<Fut1: Future, Fut2: Future> Future for TwoFutures<Fut1, Fut2> {
    type Output = Fut2::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if !*this.first_done {
            if let Poll::Ready(_) = this.first.poll(cx) {
                *this.first_done = true;
            }
        }

        if *this.first_done {
            this.second.poll(cx)
        } else {
            Poll::Pending
        }
    }
}

// async fn sleepus() {
fn sleepus() -> impl Future<Output=()> {
    // for i in 1..=10 {
    //     println!("sleepus {}", i);
    //     sleep(Duration::from_millis(500)).await;
    // }

    // SleepPrint {
    //     sleep: sleep(Duration::from_secs(3))
    // }

    TwoFutures {
        first_done: false,
        first: sleep(Duration::from_secs(3)),
        second: {
            println!("Hello TwoFutures");
            async_std::future::ready(())
        },
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("interruptus {}", i);
        sleep(Duration::from_secs(1)).await;
    }
}

#[async_std::main]
async fn main() {
    let s = spawn(sleepus());
    interruptus().await;

    s.await;
}
