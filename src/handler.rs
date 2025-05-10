use crate::cli::Cli;
use crate::detector::detect_protocol;
use crate::protocol::Protocol;
use tokio::net::TcpStream;
use tracing::{error, info, warn};

use clap::Parser;

pub async fn handle_connection(mut stream: TcpStream) -> tokio::io::Result<()> {
    let cli = Cli::parse();

    let protocol = detect_protocol(&stream).await?;

    let backend_addr = match protocol {
        Protocol::Http => cli.http.as_deref(),
        Protocol::Https => cli.https.as_deref(),
        Protocol::Ssh => cli.ssh.as_deref(),
        Protocol::Rdp => cli.rdp.as_deref(),
        Protocol::Unknown => {
            if cli.fallback.is_some() {
                warn!(
                    "unknown protocol, using fallback: {}",
                    cli.fallback.as_deref().unwrap()
                );
                cli.fallback.as_deref()
            } else {
                error!("unknown protocol, dropping connection");
                return Ok(());
            }
        }
    };
    let Some(addr) = backend_addr else {
        error!("protocol {:?} not enabled", protocol);
        return Ok(());
    };

    info!("serving with: {}", addr);

    let mut outbound = TcpStream::connect(addr).await?;

    tokio::io::copy_bidirectional(&mut stream, &mut outbound).await?;

    Ok(())
}
