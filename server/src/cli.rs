#[derive(clap::Parser)]
pub struct Cli {
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    #[clap(short, long, default_value = "easy")]
    pub difficulty: String,
}
