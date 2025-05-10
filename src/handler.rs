use crate::cli::Cli;
use crate::detector::detect_protocol;
use crate::protocol::Protocol;
use tokio::net::TcpStream;

use clap::Parser;

pub async fn handle_connection(mut inbound: TcpStream) -> tokio::io::Result<()> {
    let cli = Cli::parse();

    let mut buffer = [0u8; 1024];
    let n = inbound.peek(&mut buffer).await?;

    let protocol = detect_protocol(&buffer[..n]);

    let backend_addr = match protocol {
        Protocol::Http => cli.http,
        Protocol::Https => cli.https,
        Protocol::Ssh => cli.ssh,
        Protocol::Rdp => cli.rdp,
        Protocol::Unknown => {
            eprintln!("Unknown protocol, dropping");
            return Ok(());
        }
    };

    let mut outbound = TcpStream::connect(backend_addr).await?;

    tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await?;

    Ok(())
}
