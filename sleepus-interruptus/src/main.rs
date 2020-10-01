use async_std::task::{sleep, spawn};
use std::time::Duration;

async fn sleepus() {
    for i in 1..=10 {
        println!("sleepus {}", i);
        sleep(Duration::from_millis(500)).await;
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
