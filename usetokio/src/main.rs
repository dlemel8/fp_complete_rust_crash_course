use std::io::Error;
use std::time::Duration;
use tokio::io;
use tokio::process::Command;
use tokio::task;
use tokio::time;
use tokio::net::{TcpStream, TcpListener};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let task1 = task::spawn(echo_stdin());
    // let task2 = task::spawn(print_date());
    // task1.await??;
    // task2.await??;

    let mut listener = TcpListener::bind("127.0.0.1:8888").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        task::spawn(echo_server(socket));
    }

    Ok(())
}

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
