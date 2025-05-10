use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Address and port to listen on (e.g., 0.0.0.0:9000)
    #[arg(long, short, default_value = "0.0.0.0:5500")]
    pub listen: String,

    #[arg(long, default_value = "127.0.0.1:80")]
    pub http: String,

    #[arg(long, default_value = "127.0.0.1:443")]
    pub https: String,

    #[arg(long, default_value = "127.0.0.1:22")]
    pub ssh: String,

    #[arg(long, default_value = "127.0.0.1:3389")]
    pub rdp: String,
}
