// src/main.rs
mod cli;
mod detector;
mod handler;
mod protocol;

use clap::Parser;
use cli::Cli;
use handler::handle_connection;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let listener = TcpListener::bind(cli.listen).await?;
    info!("listening on 0.0.0.0:5500");

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("new connection from {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                error!("error handling connection from {addr}: {e}");
            }
        });
    }
}
