use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Address and port to listen on (e.g., 0.0.0.0:9000)
    #[arg(long, short, default_value = "0.0.0.0:5500")]
    pub listen: String,

    /// Optional fallback backend for unknown protocols
    #[arg(long)]
    pub fallback: Option<String>,

    /// Set http backend (ex: 127.0.0.1:80)
    #[arg(long)]
    pub http: Option<String>,

    /// Set https backend (ex: 127.0.0.1:443)
    #[arg(long)]
    pub https: Option<String>,

    /// Set ssh backend (ex: 127.0.0.1:22)
    #[arg(long)]
    pub ssh: Option<String>,

    /// Set rdp backend (ex: 127.0.0.1:3389)
    #[arg(long)]
    pub rdp: Option<String>,
}
