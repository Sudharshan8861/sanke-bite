use snake_game::{
    rng::{RngLike, Seeded},
    state::GameState,
    types::*,
};

use snake_game::systems::{Input, Time, Loop as GameLoop};

#[test]
fn test_game_state_initialization() {
    let grid = GridSize { w: 10, h: 10 };
    let rng = Seeded::new(42);
    let state = GameState::new(grid, rng);

    // Check that snake starts in center
    assert_eq!(state.snake.body.len(), 1);
    let center = Position { x: 5, y: 5 };
    assert_eq!(state.snake.body[0], center);

    // Check initial direction
    assert_eq!(state.snake.dir, Direction::Right);

    // Check initial score
    assert_eq!(state.score, 0);

    // Check game is not over initially
    assert!(!state.is_over());

    // Check that food is not on snake
    assert_ne!(state.food, state.snake.body[0]);
}

#[test]
fn test_game_state_different_grid_sizes() {
    let grids = vec![
        GridSize { w: 5, h: 5 },
        GridSize { w: 8, h: 12 },
        GridSize { w: 20, h: 15 },
    ];

    for grid in grids {
        let rng = Seeded::new(123);
        let state = GameState::new(grid, rng);

        // Snake should start in center of grid
        let expected_x = grid.w / 2;
        let expected_y = grid.h / 2;
        assert_eq!(state.snake.body[0].x, expected_x);
        assert_eq!(state.snake.body[0].y, expected_y);

        // Food should not be on snake
        assert_ne!(state.food, state.snake.body[0]);
    }
}

#[test]
fn test_seeded_rng_deterministic() {
    let mut rng1 = Seeded::new(42);
    let mut rng2 = Seeded::new(42);

    // Both should produce the same sequence
    for _ in 0..10 {
        assert_eq!(rng1.next_u32(), rng2.next_u32());
    }
}

#[test]
fn test_different_seeds_produce_different_sequences() {
    let mut rng1 = Seeded::new(42);
    let mut rng2 = Seeded::new(43);

    let first_val1 = rng1.next_u32();
    let first_val2 = rng2.next_u32();

    assert_ne!(first_val1, first_val2);
}

#[test]
fn test_seeded_rng_distribution() {
    let mut rng = Seeded::new(123);
    let mut values = [0u32; 100];
    for value in &mut values {
        *value = rng.next_u32();
    }
    let unique_values: std::collections::HashSet<u32> = values.iter().cloned().collect();
    assert!(unique_values.len() > 80, "Expected good distribution of random values");
}

#[test]
fn test_modulo_bounds_for_food_spawn() {
    let mut rng = Seeded::new(456);
    let grid_w = 20;
    let grid_h = 15;
    for _ in 0..1000 {
        let x = (rng.next_u32() as i32).rem_euclid(grid_w);
        let y = (rng.next_u32() as i32).rem_euclid(grid_h);
        assert!(x >= 0 && x < grid_w, "x should be in bounds");
        assert!(y >= 0 && y < grid_h, "y should be in bounds");
    }
}

// ---- systems (Loop) integration tests ----

struct ScriptedInput {
    directions: Vec<Direction>,
    index: usize,
}

impl ScriptedInput {
    fn new(directions: Vec<Direction>) -> Self {
        Self { directions, index: 0 }
    }
    fn advance(&mut self) {
        if self.index < self.directions.len() {
            self.index += 1;
        }
    }
}

impl Input for ScriptedInput {
    fn current_dir(&self) -> Direction {
        self.directions
            .get(self.index)
            .copied()
            .unwrap_or(Direction::Right)
    }
}

#[derive(Default)]
struct MockTime { current_tick: u64 }

impl MockTime { fn new() -> Self { Self { current_tick: 0 } } fn current(&self) -> u64 { self.current_tick } }

impl Time for MockTime {
    fn tick(&mut self) -> Tick {
        self.current_tick += 1;
        Tick(self.current_tick)
    }
}

#[test]
fn test_loop_updates_game_state() {
    let grid = GridSize { w: 10, h: 10 };
    let mut game_state = GameState::new(grid, Seeded::new(42));
    let initial_head = game_state.snake.body[0];

    let input = ScriptedInput::new(vec![Direction::Right]);
    let time = MockTime::new();
    let rng = Seeded::new(123);

    let mut loop_system: GameLoop<_, _, _> = GameLoop { input, time, rng };
    loop_system.update(&mut game_state);

    assert_eq!(
        game_state.snake.body[0],
        Position { x: initial_head.x + 1, y: initial_head.y }
    );
    assert_eq!(loop_system.time.current(), 1);
}

#[test]
fn test_loop_deterministic_sequence() {
    let grid = GridSize { w: 10, h: 10 };
    let mut game_state = GameState::new(grid, Seeded::new(42));
    let directions = vec![Direction::Right, Direction::Right, Direction::Down, Direction::Down];
    let input = ScriptedInput::new(directions);
    let time = MockTime::new();
    let rng = Seeded::new(100);
    let mut loop_system: GameLoop<_, _, _> = GameLoop { input, time, rng };
    let initial_head = game_state.snake.body[0];
    for i in 0..4 {
        loop_system.update(&mut game_state);
        loop_system.input.advance();
        assert_eq!(loop_system.time.current(), (i + 1) as u64);
    }
    assert_eq!(
        game_state.snake.body[0],
        Position { x: initial_head.x + 2, y: initial_head.y + 2 }
    );
}

#[test]
fn test_mock_time_advances() {
    let mut time = MockTime::new();
    assert_eq!(time.current(), 0);
    let tick1 = time.tick();
    assert_eq!(tick1.0, 1);
    assert_eq!(time.current(), 1);
    let tick2 = time.tick();
    assert_eq!(tick2.0, 2);
    assert_eq!(time.current(), 2);
}

#[test]
fn test_scripted_input_returns_directions() {
    let input = ScriptedInput::new(vec![Direction::Up, Direction::Down, Direction::Left]);
    assert_eq!(input.current_dir(), Direction::Up);
}

#[test]
fn test_paused_state_prevents_movement_via_loop() {
    let grid = GridSize { w: 10, h: 10 };
    let mut game_state = GameState::new(grid, Seeded::new(1));
    let initial_head = game_state.snake.body[0];
    let input = ScriptedInput::new(vec![Direction::Right]);
    let time = MockTime::new();
    let rng = Seeded::new(2);
    let mut loop_system: GameLoop<_, _, _> = GameLoop { input, time, rng };
    game_state.pause();
    loop_system.update(&mut game_state);
    assert_eq!(game_state.snake.body[0], initial_head);
    assert!(game_state.is_paused());
}

#[test]
fn test_resume_allows_movement_again_via_loop() {
    let grid = GridSize { w: 10, h: 10 };
    let mut game_state = GameState::new(grid, Seeded::new(1));
    let initial_head = game_state.snake.body[0];
    let input = ScriptedInput::new(vec![Direction::Right]);
    let time = MockTime::new();
    let rng = Seeded::new(2);
    let mut loop_system: GameLoop<_, _, _> = GameLoop { input, time, rng };
    game_state.pause();
    game_state.resume();
    loop_system.update(&mut game_state);
    assert_eq!(
        game_state.snake.body[0],
        Position { x: initial_head.x + 1, y: initial_head.y }
    );
}

// ---- state reset invariants ----

#[test]
fn test_reset_restores_invariants() {
    let grid = GridSize { w: 12, h: 8 };
    let rng = Seeded::new(99);
    let mut state = GameState::new(grid, rng.clone());
    state.score = 5;
    state.snake.dir = Direction::Down;
    let head = state.snake.body[0];
    state.snake.body.push_back(Position { x: head.x - 1, y: head.y });
    // Mark over then reset
    // RunState is internal; black-box via public API resets invariants
    state.reset(rng.clone());
    let center = Position { x: grid.w / 2, y: grid.h / 2 };
    assert_eq!(state.grid, grid);
    assert_eq!(state.score, 0);
    assert_eq!(state.snake.dir, Direction::Right);
    assert_eq!(state.snake.body.len(), 1);
    assert_eq!(state.snake.body[0], center);
    assert_ne!(state.food, center);
}
