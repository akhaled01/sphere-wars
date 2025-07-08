# Sphere Wars 🎮

A fast-paced multiplayer FPS maze game built with Rust and the Bevy game engine. Battle other players in procedurally generated 3D mazes using server-authoritative networking for competitive gameplay.

## 🚀 Features

### Multiplayer Combat
- **Real-time FPS Combat**: Server-authoritative shooting system with hitscan weapons
- **Up to 8 Players**: Competitive multiplayer matches with 2-8 players
- **Health & Damage System**: 100 HP with 50 damage per hit (2-shot kills)
- **Death & Respawn**: Instant respawn at random maze spawn points
- **Kill Tracking**: Real-time kill/death statistics
- **Player Colors**: Unique server-assigned colors for each player

### 3D Maze Combat
- **Procedural Mazes**: Server-generated 12x12 mazes with multiple spawn points
- **First-Person Shooting**: Screen-center crosshair shooting with ray-casting
- **Wall Collision**: Precise ray-wall intersection for realistic bullet physics
- **Tactical Positioning**: Use maze walls for cover and strategic positioning
- **Real-time Minimap**: Live player positions and maze layout

### Network Architecture
- **UDP Networking**: Low-latency UDP client-server architecture
- **Server Authority**: All combat calculations handled server-side
- **Real-time Sync**: Player positions, health, and game state synchronized
- **Graceful Shutdown**: Coordinated client disconnection on server shutdown
- **Connection Testing**: Pre-game server connectivity validation

## 🎯 How to Play

### Controls
- **Movement**: `W` `A` `S` `D` - Move forward, left, backward, right
- **Camera**: `Mouse` - Look around (pitch and yaw)
- **Shooting**: `Left Click` - Fire weapon (1 shot per second)
- **Respawn**: `R` - Manual respawn when dead
- **Navigation**: Use the minimap in the bottom-right corner to track players

### Objective
Eliminate other players in intense FPS combat within the maze. Use walls for cover, track enemies on the minimap, and dominate the leaderboard with your kill/death ratio.

## 🛠️ Installation & Setup

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Quick Start
```bash
# Clone the repository
git clone https://github.com/akhaled01/sphere-wars.git
cd sphere-wars

# Build the project
make

# Start the server (Terminal 1)
war-server

# Start client(s) (Terminal 2+)
war-client
```

### Build Commands
```bash
# Build all components and initialize font files
make

# Build individual components
cargo build --release --bin server
cargo build --release --bin client
```

### Server Configuration
- **Host**: 127.0.0.1 (localhost)
- **Port**: 8080 (UDP)
- **Maze Size**: 12x12 with randomized spawn points

## 🏗️ Technical Architecture

### Client-Server Model
- **UDP Networking**: Custom UDP protocol for real-time multiplayer
- **Server Authority**: All game logic, combat, and state managed server-side
- **Client Prediction**: Smooth movement with server reconciliation
- **Message System**: JSON-based client-server communication

### Core Systems
- **Shooting System**: Server-authoritative hitscan with ray-box intersection
- **Maze Generation**: Procedural maze generation with spawn point allocation
- **Player Management**: Unique player IDs, colors, and spawn point tracking
- **Health System**: Damage calculation, death detection, and respawn logic
- **Physics & Collision**: Precise collision detection for walls and player interactions
- **Camera System**: First-person camera with mouse look controls
- **Minimap System**: Real-time synchronized minimap with player positions

### Network Messages
- **Client → Server**: JoinGame, LeaveGame, PlayerMove, PlayerShoot, Respawn
- **Server → Client**: GameJoined, GameState, PlayerUpdate, PlayerShot, PlayerDied, GameEnded

### Performance Optimizations
- **60+ FPS Target**: Optimized rendering and lighting systems
- **Efficient Networking**: Minimal message overhead with UDP
- **Memory Management**: Proper resource cleanup and spawn point reuse
- **Ray Casting**: Optimized wall intersection algorithms

## 🎨 Game Design

### Combat Mechanics
- **Weapon System**: Hitscan weapons with 1 shot/second fire rate
- **Damage Model**: 50 damage per hit, 100 HP total (2-shot kills)
- **Hit Detection**: Server-side ray-box intersection for walls and spheres
- **Visual Feedback**: Hit effects (orange spheres) and damage overlays
- **Death System**: Immediate death at 0 HP with respawn mechanics

### Maze Features
- **Scale**: 4-unit tiles with 2-unit wide corridors for tactical movement
- **Dimensions**: 12x12 maze generation with multiple spawn points
- **Walls**: 8-unit tall walls for cover and strategic positioning
- **Materials**: High-contrast materials for clear visibility
- **Spawn Points**: Randomized spawn locations to prevent camping

## 🔧 Development

### Project Structure
```
sphere-wars/
├── client/         # Bevy-based game client
│   ├── src/
│   │   ├── components/ # Game components (Player, Weapon, etc.)
│   │   ├── plugins/    # Network and world plugins
│   │   ├── systems/    # Game systems
│   │   │   ├── player/ # Movement, camera, shooting
│   │   │   ├── world/  # Maze rendering, lighting
│   │   │   └── ui/     # Minimap, death screen, FPS
│   │   └── main.rs     # Client entry point
├── server/         # UDP game server
│   ├── src/
│   │   ├── server.rs   # Game server logic
│   │   ├── utils.rs    # Networking utilities
│   │   └── main.rs     # Server entry point
├── shared/         # Shared data structures
│   ├── src/
│   │   ├── messages.rs # Network message definitions
│   │   ├── player.rs   # Player data structures
│   │   ├── maze.rs     # Maze generation
│   │   └── lib.rs      # Shared library
└── Makefile        # Build system
```

### Key Technologies
- **Rust**: Systems programming language for performance and safety
- **Bevy**: Modern ECS-based game engine for client
- **Tokio**: Async runtime for server networking
- **bincode**: Binary serialization for network messages
- **UDP Networking**: Custom protocol for real-time multiplayer
- **Ray Casting**: 3D intersection algorithms for shooting

## 🤝 Contributing

We welcome contributions! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📊 Performance

- **Target FPS**: 60+ FPS on modern hardware
- **Network Latency**: Optimized UDP for minimal lag
- **Memory Usage**: Efficient resource management and cleanup
- **Platform**: Cross-platform support (Windows, macOS, Linux)
- **Concurrent Players**: Supports up to 8 simultaneous players

## 🎮 Multiplayer Features

### ✅ Implemented
- **Real-time Combat**: Server-authoritative FPS shooting
- **Player vs Player**: Competitive sphere battles in maze environments
- **Health & Damage**: Complete combat system with death/respawn
- **Live Statistics**: Kill/death tracking and player colors
- **Synchronized Gameplay**: Real-time player positions and game state
- **Graceful Networking**: Connection testing and coordinated shutdown

### 🚧 Future Enhancements
- **Team Modes**: Cooperative and team-based game modes
- **Weapon Variety**: Different weapon types and damage models
- **Power-ups**: Collectible items and temporary abilities
- **Leaderboards**: Persistent statistics and rankings
- **Spectator Mode**: Watch ongoing matches
- **Custom Maps**: User-generated maze configurations

## 📝 License

This project is open source. See the LICENSE file for details.

## 🙏 Acknowledgments

- Built with the [Bevy](https://bevyengine.org/) game engine
- Inspired by Maze Runner and DOOM
