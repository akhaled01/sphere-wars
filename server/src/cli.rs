use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum MazeDifficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for MazeDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeDifficulty::Easy => write!(f, "Easy"),
            MazeDifficulty::Medium => write!(f, "Medium"),
            MazeDifficulty::Hard => write!(f, "Hard"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "mazey-server")]
#[command(about = "A multiplayer maze game server")]
#[command(version = "0.1.0")]
pub struct Args {
    /// Host address to bind the server to
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port number to bind the server to
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Maze difficulty level
    #[arg(short = 'd', long, default_value = "medium")]
    pub maze_difficulty: MazeDifficulty,
}
