use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Address and port to listen on (e.g., 0.0.0.0:9000)
    #[arg(long, short, default_value = "0.0.0.0:5500")]
    pub listen: String,

    #[arg(long)]
    pub http: Option<String>,

    #[arg(long)]
    pub https: Option<String>,

    #[arg(long)]
    pub ssh: Option<String>,

    #[arg(long)]
    pub rdp: Option<String>,
}
