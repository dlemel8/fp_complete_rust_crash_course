use std::io::Error;
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}
