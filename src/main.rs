// src/main.rs
mod cli;
mod detector;
mod handler;
mod protocol;

use clap::Parser;
use cli::Cli;
use handler::handle_connection;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let cli = Cli::parse();

    let listener = TcpListener::bind(cli.listen).await?;
    println!("Listening on 0.0.0.0:5500");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection from {addr}: {e}");
            }
        });
    }
}
