#[derive(clap::Parser)]
pub struct Cli {
    #[clap(long, default_value = "127.0.0.1", help = "Server host address")]
    pub host: String,
    #[clap(short, long, default_value = "8080", help = "Server port")]
    pub port: u16,
    #[clap(
        short,
        long,
        default_value = "medium",
        help = "Maze difficulty level",
        long_help = "Maze difficulty affects maze complexity:\n  easy   - More connections, fewer dead ends (25% extra connections, 40% dead end removal)\n  medium - Balanced maze (15% extra connections, 20% dead end removal)\n  hard   - Minimal connections, more dead ends (5% extra connections, no dead end removal)"
    )]
    pub difficulty: String,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        match self.difficulty.as_str() {
            "easy" | "medium" | "hard" => Ok(()),
            _ => Err(format!(
                "Invalid difficulty '{}'. Valid options are: easy, medium, hard",
                self.difficulty
            )),
        }
    }
}
