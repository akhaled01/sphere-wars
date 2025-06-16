# Tank Wars 🎮

A thrilling multiplayer 3D maze game built with Rust and the Bevy game engine. Navigate through procedurally generated mazes in your tank while competing against other players in intense tactical combat.

## 🚀 Features

### Core Gameplay
- **3D Maze Navigation**: Explore complex procedurally generated mazes with your tank
- **First-Person Perspective**: Immersive camera controls with full pitch and yaw movement
- **Real-time Minimap**: Navigate with confidence using the synchronized minimap that shows your exact position and the maze layout
- **Smooth Controls**: Responsive WASD movement with mouse look controls

### Technical Highlights
- **Bevy Engine**: Built on Rust's powerful Bevy game engine for high performance
- **Procedural Generation**: Each maze is uniquely generated using advanced algorithms
- **Optimized Rendering**: Efficient 3D rendering with proper lighting and materials
- **60 FPS Performance**: Smooth gameplay optimized for modern hardware

### Visual Features
- **Modern 3D Graphics**: High-quality materials and lighting effects
- **Tank Model**: Detailed 3D tank with proper collision detection
- **Dynamic Lighting**: Atmospheric lighting system for immersive gameplay
- **UI Elements**: Clean HUD with FPS counter and minimap overlay

## 🎯 How to Play

### Controls
- **Movement**: `W` `A` `S` `D` - Move forward, left, backward, right
- **Camera**: `Mouse` - Look around (pitch and yaw)
- **Navigation**: Use the minimap in the bottom-right corner to orient yourself

### Objective
Navigate through the maze to reach objectives while avoiding or engaging other players in tactical combat.

## 🛠️ Installation & Setup

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Quick Start
```bash
# Clone the repository
git clone https://github.com/akhaled01/tank-go.git
cd tank-go

# Run the game
cargo run
```

### Build for Release
```bash
cargo build --release
```

## 🏗️ Technical Architecture

### Core Systems
- **Maze Generation**: Advanced procedural maze generation with configurable difficulty
- **Physics & Collision**: Precise collision detection for walls and player interactions
- **Camera System**: Smooth first-person camera with clamped pitch control
- **Minimap System**: Real-time synchronized minimap with accurate player tracking
- **Lighting System**: Optimized lighting for performance and visual quality

### Performance Optimizations
- **Shared Resources**: Single maze generation shared between 3D world and minimap
- **Efficient Rendering**: Optimized mesh generation and material usage
- **System Scheduling**: Proper dependency management for smooth gameplay

## 🎨 Game Design

### Maze Features
- **Scale**: 6-unit wide corridors for comfortable tank navigation
- **Dimensions**: 12x12 node generation creating 37x37 matrix layouts
- **Walls**: 18-unit tall walls for immersive maze experience
- **Materials**: High-contrast materials for clear visibility

### Visual Design
- **Minimap Colors**:
  - 🟢 **Green**: Pathways/corridors
  - 🔵 **Blue-White**: Walls
  - 🔴 **Red**: Player position
- **3D World**: Realistic materials with proper lighting and shadows

## 🔧 Development

### Project Structure
```
src/
├── components/     # Game components and resources
├── plugins/        # Bevy plugins for modular architecture
└── systems/        # Game systems (world, UI, player, etc.)
    ├── world/      # World generation and rendering
    │   ├── maze.rs # Maze generation and rendering
    │   └── ui/     # UI systems (minimap, FPS counter)
    └── player/     # Player movement and camera controls
```

### Key Technologies
- **Rust**: Systems programming language for performance and safety
- **Bevy**: Modern ECS-based game engine
- **3D Graphics**: Mesh rendering with PBR materials
- **Procedural Generation**: Custom maze generation algorithms

## 🤝 Contributing

We welcome contributions! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📊 Performance

- **Target FPS**: 60 FPS on modern hardware
- **Memory Usage**: Optimized for efficient memory management
- **Platform**: Cross-platform support (Windows, macOS, Linux)

## 🎮 Multiplayer Features (Coming Soon)

- **Network Play**: Real-time multiplayer combat
- **Player vs Player**: Tactical tank battles in maze environments
- **Team Modes**: Cooperative and competitive game modes
- **Leaderboards**: Track your performance and compete globally

## 📝 License

This project is open source. See the LICENSE file for details.

## 🙏 Acknowledgments

- Built with the [Bevy](https://bevyengine.org/) game engine
- Tank model and assets from various open-source contributors
- Inspired by classic maze and tank games
