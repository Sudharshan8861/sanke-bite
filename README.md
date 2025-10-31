# 🐍 Snake Game

<div align="center">

**A classic Snake game built with Rust and egui, featuring clean architecture, deterministic gameplay, and comprehensive testing.**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![egui](https://img.shields.io/badge/egui-0.27-green.svg)](https://github.com/emilk/egui)

</div>

---

## 📋 Table of Contents

- [Features](#-features)
- [Architecture](#-architecture)
- [Project Structure](#-project-structure)
- [Installation](#-installation)
- [Usage](#-usage)
- [Controls](#-controls)
- [Testing](#-testing)
- [Development](#-development)
- [Documentation](#-documentation)
- [License](#-license)

---

## ✨ Features

- 🎮 **Classic Gameplay**: Traditional Snake game mechanics with smooth controls
- 🎯 **Deterministic RNG**: Reproducible gameplay using seeded random number generation
- 🏗️ **Clean Architecture**: Separation of concerns with ports & adapters pattern
- 🧪 **Comprehensive Testing**: Unit, integration, and property tests
- 🎨 **Modern UI**: Built with egui for cross-platform native applications
- 🔄 **Extensible Design**: Easy to add new frontends (TUI, Web) or features
- ⚡ **Performance**: Efficient game loop with minimal overhead

---

## 🏗️ Architecture

This project follows a **clean architecture** pattern with clear separation between:

- **Engine Core**: Pure game logic, UI-agnostic, fully testable
- **Ports & Adapters**: Trait-based interfaces for Input, Time, and RNG
- **UI Layer**: egui frontend that adapts to the engine via traits

The design enables:
- ✅ 90%+ test coverage with fast, headless tests
- ✅ Easy frontend swapping (egui → TUI → Web)
- ✅ Deterministic gameplay for testing and replay
- ✅ Incremental feature development

### Key Design Patterns

- **Ports & Adapters**: `Input` and `Time` traits decouple UI from game logic
- **Dependency Injection**: RNG, input, and time are injected via generics
- **Pure Functions**: Core game rules are pure and easily testable
- **Deterministic Tests**: Seeded RNG ensures reproducible test scenarios

---

## 📁 Project Structure

```
snake-game/
├── 📄 Cargo.toml              # Project manifest and dependencies
├── 📄 Cargo.lock              # Dependency lock file
├── 📄 Makefile.toml           # Development tasks (make check, make test, etc.)
├── 📄 README.md               # This file
├── 📄 architecture.md         # Detailed architecture documentation
├── 📄 refactor.md             # Refactoring plan and progress
│
├── 📂 src/                    # Source code
│   ├── 📄 main.rs             # Application entry point (egui app)
│   ├── 📄 lib.rs              # Library root (exports all modules)
│   │
│   ├── 🎯 Engine Core         # Pure game logic (UI-agnostic)
│   │   ├── 📄 types.rs        # Core types (Position, Direction, GridSize, Tick)
│   │   ├── 📄 rng.rs          # RNG trait and Seeded implementation
│   │   ├── 📄 state.rs        # GameState and Snake structures
│   │   ├── 📄 rules.rs        # Game rules (movement, collisions, food spawning)
│   │   └── 📄 systems.rs       # Ports & Adapters (Input, Time, Loop)
│   │
│   └── 🎨 UI Layer            # egui frontend adapters
│       ├── 📄 input.rs         # Keyboard input adapter (maps egui → Direction)
│       └── 📄 render.rs        # Rendering module (grid, snake, food, HUD)
│
├── 📂 tests/                  # Integration and external tests
│   ├── 📄 rules_tests.rs       # Comprehensive rules testing
│   └── 📄 integration_tests.rs # End-to-end integration tests
│
└── 📂 target/                  # Build artifacts (generated)
```

### Module Responsibilities

| Module | Purpose | UI Dependency |
|--------|---------|---------------|
| `types` | Core domain types | ❌ None |
| `rng` | Random number generation abstraction | ❌ None |
| `state` | Game state management | ❌ None |
| `rules` | Game rules and logic | ❌ None |
| `systems` | Ports & Adapters (traits) | ❌ None |
| `input` | egui keyboard input → Direction | ✅ egui |
| `render` | Rendering game state | ✅ egui |
| `main` | Application setup and loop | ✅ egui |

---

## 🚀 Installation

### Prerequisites

- **Rust** (1.70 or later)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo** (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd snake-game

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Development Tools

Optional tools for enhanced development:

```bash
# Install cargo-make for development tasks
cargo install cargo-make

# Use Makefile.toml tasks
cargo make check    # Format check + clippy
cargo make test      # Run all tests
cargo make build     # Build the project
```

---

## 🎮 Usage

### Running the Game

```bash
cargo run
```

Or build and run the release binary:

```bash
cargo build --release
./target/release/snake_game
```

### Controls

| Key | Action |
|-----|--------|
| `↑` `↓` `←` `→` | Move snake (Arrow keys) |
| `W` `A` `S` `D` | Move snake (WASD keys) |
| `Space` | Pause/Resume game |
| `R` | Reset game |

### Gameplay

- 🐍 **Objective**: Control the snake to eat food and grow longer
- ⚠️ **Game Over**: Hitting walls or your own body ends the game
- 📈 **Score**: Increases by 1 for each food eaten
- 🎯 **Prevention**: 180-degree reversals are prevented to avoid instant collisions

---

## 🧪 Testing

The project includes comprehensive test coverage:

### Run All Tests

```bash
cargo test
```

### Run Specific Test Suites

```bash
# Unit tests (engine core)
cargo test --lib

# Integration tests
cargo test --test rules_tests
cargo test --test integration_tests

# Test with output
cargo test -- --nocapture
```

### Test Coverage

- ✅ **Unit Tests**: Core types, RNG, state management, rules
- ✅ **Integration Tests**: End-to-end game scenarios
- ✅ **Property Tests**: Food spawning safety, movement validation
- ✅ **Edge Cases**: Wall collisions, self-collisions, boundary conditions

### Test Examples

```rust
// Example: Food never spawns on snake
#[test]
fn food_never_spawns_on_snake() {
    let grid = GridSize { w: 8, h: 8 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());
    for _ in 0..1000 {
        snake_game::rules::step(&mut g, &mut rng);
        assert!(!g.snake.body.iter().any(|&p| p == g.food));
    }
}
```

---

## 🔧 Development

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check

# Run clippy (linter)
cargo clippy --all-targets --all-features -- -D warnings
```

### Development Workflow

1. **Make changes** to source code
2. **Run tests** to ensure nothing breaks
3. **Format code** with `cargo fmt`
4. **Check linting** with `cargo clippy`
5. **Commit** changes

### Adding New Features

The architecture supports incremental feature development:

1. **Engine Features**: Add to `rules.rs` or `state.rs`
2. **UI Features**: Extend `input.rs` or `render.rs`
3. **New Frontends**: Implement `Input` and `Time` traits

See `architecture.md` for detailed development guidelines.

---

## 📚 Documentation

### Architecture Documentation

- **[architecture.md](architecture.md)**: Complete architecture overview, design patterns, and roadmap
- **[refactor.md](refactor.md)**: Refactoring plan and progress tracking

### Code Documentation

Generate API documentation:

```bash
# Generate docs
cargo doc --open

# Generate docs for all dependencies
cargo doc --all-features --open
```

### Key Concepts

- **Ports & Adapters**: `systems.rs` defines the interfaces between UI and engine
- **Deterministic RNG**: `rng.rs` provides seeded random number generation
- **Game Loop**: `systems::Loop` orchestrates input, rules, and time updates
- **Clean Separation**: Engine modules have no UI dependencies

---

## 🎯 Roadmap

The project follows an incremental development approach with 20 planned PRs:

- ✅ **PRs 1-11**: Core engine + egui minimal app (Completed)
- 🔄 **PRs 12-15**: Settings, persistence, wrap-walls feature
- 📋 **PRs 16-20**: Power-ups, replay system, TUI frontend, polish

See `architecture.md` for the complete roadmap.

---

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. ✅ All tests pass
2. ✅ Code is formatted (`cargo fmt`)
3. ✅ No clippy warnings (`cargo clippy -D warnings`)
4. ✅ Tests are added for new features
5. ✅ Documentation is updated

---

## 📄 License

This project is licensed under the **MIT License** - see the LICENSE file for details.

---

## 🙏 Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - A fast, portable immediate mode GUI library
- Inspired by classic Snake game mechanics
- Architecture patterns from clean architecture and ports & adapters

---

<div align="center">

**Made with ❤️ and Rust**

[Report Bug](https://github.com/yourusername/snake-game/issues) · [Request Feature](https://github.com/yourusername/snake-game/issues) · [Documentation](architecture.md)

</div>

