use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::io::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time;
use std::ops::Deref;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    args.next(); // exe name

    let mut runtime = tokio::runtime::Runtime::new()?;
    let local = tokio::task::LocalSet::new();
    local.block_on(&mut runtime, count_lines(args));

    // count_lines(args).await?;

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // let task1 = task::spawn(echo_stdin());
//     // let task2 = task::spawn(print_date());
//     // task1.await??;
//     // task2.await??;
//
//     // let mut listener = TcpListener::bind("127.0.0.1:8888").await?;
//     // loop {
//     //     let (socket, _) = listener.accept().await?;
//     //     task::spawn(echo_server(socket));
//     // }
//
//     let mut args = std::env::args();
//     args.next(); // exe name
//     count_lines(args).await?;
//
//     Ok(())
// }

async fn echo_stdin() -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}

async fn print_date() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        Command::new("date").spawn()?.await?;
    }
}

async fn echo_server(socket: TcpStream) -> io::Result<()> {
    let (mut recv, mut send) = io::split(socket);
    io::copy(&mut recv, &mut send).await?;
    Ok(())
}

async fn count_lines(paths: std::env::Args) -> io::Result<()> {
    let mut tasks = vec![];
    let count = Arc::new(Mutex::new(0u32));

    for path in paths {
        let count_shared = count.clone();
        tasks.push(tokio::spawn(async move {
            let mut local_count = 0u32;
            let file = io::BufReader::new(tokio::fs::File::open(path).await?);
            let mut lines = file.lines();
            while let Some(_) = lines.next_line().await? {
                local_count += 1;
            }

            let mut locked = count_shared.lock().await;
            *locked += local_count;

            Ok(()) as Result<(), std::io::Error>
        }));
    }

    for task in tasks {
        task.await??;
    }

    println!("lines count is {}", count.lock().await);
    Ok(())
}

async fn count_lines_local(paths: std::env::Args) -> io::Result<()> {
    let mut tasks = vec![];
    let count = Rc::new(RefCell::new(0u32));

    for path in paths {
        let count_shared = count.clone();
        tasks.push(tokio::task::spawn_local(async move {
            let mut local_count = 0u32;
            let file = io::BufReader::new(tokio::fs::File::open(path).await?);
            let mut lines = file.lines();
            while let Some(_) = lines.next_line().await? {
                local_count += 1;
            }

            *count_shared.deref().borrow_mut() += local_count;
            Ok(()) as Result<(), std::io::Error>
        }));
    }

    for task in tasks {
        task.await??;
    }

    println!("lines count is {}", count.deref().borrow());
    Ok(())
}
