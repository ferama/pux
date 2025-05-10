// src/main.rs
mod cli;
mod detector;
mod handler;
mod protocol;

use std::io::ErrorKind;

use clap::{CommandFactory, Parser};
use cli::Cli;
use handler::handle_connection;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    // At least one backend must be provided
    if cli.http.is_none() && cli.https.is_none() && cli.ssh.is_none() && cli.rdp.is_none() {
        println!("At least one backend must be configured\n");
        let help = Cli::command().render_help();
        println!("{}", help.ansi());

        std::process::exit(1);
    }

    let listener = TcpListener::bind(cli.listen.clone()).await?;
    info!("listening on {}", cli.listen);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("new connection from {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                match e.kind() {
                    ErrorKind::UnexpectedEof
                    | ErrorKind::ConnectionReset
                    | ErrorKind::BrokenPipe
                    | ErrorKind::ConnectionAborted => {
                        info!("connection from {addr} closed");
                    }
                    _ => {
                        error!("error handling connection from {addr}: {e}");
                    }
                }
            }
        });
    }
}
