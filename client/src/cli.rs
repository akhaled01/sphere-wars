use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Server host address
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Server port
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Player name
    #[arg(short, long)]
    pub name: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
