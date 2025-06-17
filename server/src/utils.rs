use crate::cli::Args;

pub fn print_info(args: Args) {
    println!("🎮 Mazey Game Server Starting...");
    println!("📍 Host: {}", args.host);
    println!("🔌 Port: {}", args.port);
    println!("🧩 Maze Difficulty: {}", args.maze_difficulty);
    println!("🚀 Server configured successfully!");
}