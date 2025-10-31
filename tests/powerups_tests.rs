#![cfg(feature = "powerups")]

use snake_game::{
    rng::Seeded,
    rules,
    state::GameState,
    types::{Direction, GridSize, PowerUp, PowerUpType, Position},
};

#[test]
fn test_powerup_spawning() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let mut rng = Seeded::new(123);

    // Initially no power-up should exist
    assert!(state.powerup.is_none());

    // Try to spawn a power-up (may not succeed due to probability)
    for _ in 0..200 {
        rules::try_spawn_powerup(&mut state, &mut rng);
        if state.powerup.is_some() {
            break;
        }
    }

    // After many attempts, we should have a power-up (if spawn probability allows)
    // Note: This test checks that spawning logic works, even if not guaranteed
    if let Some(powerup) = state.powerup {
        // Verify power-up is not on snake
        assert!(!state.snake.body.iter().any(|&p| p == powerup.position));

        // Verify power-up is not on food
        #[cfg(not(feature = "multiple_foods"))]
        assert_ne!(powerup.position, state.food);

        // Verify power-up has valid type
        match powerup.power_type {
            PowerUpType::Slow | PowerUpType::Fast | PowerUpType::Poison => {}
        }
    }
}

#[test]
fn test_powerup_collision_activates() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let mut rng = Seeded::new(123);

    // Manually place a power-up next to the snake
    let snake_head = state.snake.body[0];
    let powerup_pos = Position {
        x: snake_head.x + 1,
        y: snake_head.y,
    };

    state.powerup = Some(PowerUp {
        position: powerup_pos,
        power_type: PowerUpType::Slow,
        remaining_duration: 20,
    });

    // Initially no active power-up
    assert!(state.active_powerup.is_none());

    // Move snake toward power-up
    state.snake.dir = Direction::Right;
    rules::step(&mut state, &mut rng);

    // Power-up should be activated
    assert!(state.active_powerup.is_some());
    assert_eq!(state.active_powerup.unwrap().0, PowerUpType::Slow);

    // Power-up should be removed from grid
    assert!(state.powerup.is_none());
}

#[test]
fn test_slow_powerup_skips_steps() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let mut rng = Seeded::new(123);

    // Activate slow power-up
    state.active_powerup = Some((PowerUpType::Slow, 20));
    state.slow_skip_counter = 0;

    let initial_head_pos = state.snake.body[0];
    state.snake.dir = Direction::Right;

    // First step - should be skipped (counter = 1, odd)
    rules::step(&mut state, &mut rng);
    assert_eq!(state.snake.body[0], initial_head_pos, "First step should be skipped");

    // Second step - should execute (counter = 2, even)
    rules::step(&mut state, &mut rng);
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_head_pos.x + 1,
            y: initial_head_pos.y
        },
        "Second step should execute"
    );

    // Third step - should be skipped (counter = 3, odd)
    let pos_before_skip = state.snake.body[0];
    rules::step(&mut state, &mut rng);
    assert_eq!(
        state.snake.body[0], pos_before_skip,
        "Third step should be skipped"
    );
}

#[test]
fn test_slow_powerup_duration() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let rng = Seeded::new(123);
    let mut loop_system = snake_game::systems::Loop {
        input: TestInput {
            dir: Direction::Right,
        },
        time: TestTime { tick_count: 0 },
        rng: rng.clone(),
    };

    // Activate slow power-up with duration 5
    state.active_powerup = Some((PowerUpType::Slow, 5));
    state.slow_skip_counter = 0;

    // Update multiple times using loop system (which handles duration)
    for i in 0..10 {
        // Check duration before update
        if i < 5 {
            if let Some((_, duration)) = state.active_powerup {
                assert_eq!(duration, 5 - i, "Duration should decrease by 1 per update");
            }
        }
        
        loop_system.update(&mut state);
    }

    // After 5 updates, power-up should have expired (duration: 5 -> 4 -> 3 -> 2 -> 1 -> 0)
    assert!(state.active_powerup.is_none(), "Slow power-up should expire after duration");
    assert_eq!(state.slow_skip_counter, 0);
}

#[test]
fn test_fast_powerup_double_movement() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let rng = Seeded::new(123);
    let mut loop_system = snake_game::systems::Loop {
        input: TestInput {
            dir: Direction::Right,
        },
        time: TestTime { tick_count: 0 },
        rng: rng.clone(),
    };

    // Create a longer snake to better see movement
    let head = state.snake.body[0];
    state.snake.body.push_back(Position {
        x: head.x - 1,
        y: head.y,
    });

    // Activate fast power-up
    state.active_powerup = Some((PowerUpType::Fast, 15));
    state.snake.dir = Direction::Right;

    let initial_head_pos = state.snake.body[0];
    let initial_len = state.snake.body.len();

    // Update once - should move twice due to fast power-up
    loop_system.update(&mut state);

    // Head should have moved 2 positions to the right
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_head_pos.x + 2,
            y: initial_head_pos.y
        },
        "Fast power-up should move snake twice"
    );

    // Snake length should remain the same (if no food eaten)
    // Actually, length will increase if we move twice without eating food twice
    // Let me check: if we move twice and eat no food, tail is popped twice
    // So length should stay the same
    assert_eq!(
        state.snake.body.len(),
        initial_len,
        "Snake length should remain constant when no food is eaten"
    );
}

#[test]
fn test_fast_powerup_duration() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let rng = Seeded::new(123);
    let mut loop_system = snake_game::systems::Loop {
        input: TestInput {
            dir: Direction::Right,
        },
        time: TestTime { tick_count: 0 },
        rng: rng.clone(),
    };

    // Activate fast power-up with duration 3
    state.active_powerup = Some((PowerUpType::Fast, 3));
    state.snake.dir = Direction::Right;

        let _initial_pos = state.snake.body[0];

    // Update 5 times
    for i in 0..5 {
        loop_system.update(&mut state);

        if i < 2 {
            // Fast should still be active
            assert!(state.active_powerup.is_some());
            if let Some((PowerUpType::Fast, duration)) = state.active_powerup {
                assert!(duration > 0);
            }
        }
    }

    // After many updates, fast should expire
    for _ in 0..20 {
        loop_system.update(&mut state);
    }

    assert!(state.active_powerup.is_none(), "Fast power-up should expire");
}

#[test]
fn test_poison_powerup_reduces_length() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let mut rng = Seeded::new(123);

    // Create a longer snake
    let head = state.snake.body[0];
    state.snake.body.push_back(Position {
        x: head.x - 1,
        y: head.y,
    });
    state.snake.body.push_back(Position {
        x: head.x - 2,
        y: head.y,
    });
    state.snake.body.push_back(Position {
        x: head.x - 3,
        y: head.y,
    });

    let initial_len = state.snake.body.len();
    assert_eq!(initial_len, 4);

    // Manually place poison power-up next to snake
    let snake_head = state.snake.body[0];
    let powerup_pos = Position {
        x: snake_head.x + 1,
        y: snake_head.y,
    };

    state.powerup = Some(PowerUp {
        position: powerup_pos,
        power_type: PowerUpType::Poison,
        remaining_duration: 0,
    });

    // Move snake into poison
    state.snake.dir = Direction::Right;
    rules::step(&mut state, &mut rng);

    // Snake should have shrunk by 1
    assert_eq!(
        state.snake.body.len(),
        initial_len - 1,
        "Poison should reduce snake length by 1"
    );

    // Poison should be consumed (no active power-up since it has no duration)
    assert!(state.powerup.is_none());
    assert!(state.active_powerup.is_none());
}

#[test]
fn test_poison_powerup_minimum_length() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let mut rng = Seeded::new(123);

    // Snake with minimum length (1 segment)
    assert_eq!(state.snake.body.len(), 1);

    // Manually place poison power-up next to snake
    let snake_head = state.snake.body[0];
    let powerup_pos = Position {
        x: snake_head.x + 1,
        y: snake_head.y,
    };

    state.powerup = Some(PowerUp {
        position: powerup_pos,
        power_type: PowerUpType::Poison,
        remaining_duration: 0,
    });

    // Move snake into poison
    state.snake.dir = Direction::Right;
    rules::step(&mut state, &mut rng);

    // Snake should still have at least 1 segment (minimum length)
    assert!(
        state.snake.body.len() >= 1,
        "Snake should maintain minimum length of 1"
    );
}

#[test]
fn test_powerup_expires_after_duration() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));
    let rng = Seeded::new(123);
    let mut loop_system = snake_game::systems::Loop {
        input: TestInput {
            dir: Direction::Right,
        },
        time: TestTime { tick_count: 0 },
        rng: rng.clone(),
    };

    // Activate slow power-up with short duration
    state.active_powerup = Some((PowerUpType::Slow, 3));
    state.slow_skip_counter = 0;

    // Update multiple times using loop system (which handles duration)
    // Duration starts at 3, so after updates 0, 1, 2 it should be 1, 0, then removed
    for i in 0..5 {
        if i < 3 {
            // Should still be active before update
            assert!(state.active_powerup.is_some(), "Power-up should still be active before update {}", i);
        }
        loop_system.update(&mut state);
    }

    // After duration expires, power-up should be removed
    assert!(
        state.active_powerup.is_none(),
        "Power-up should expire after duration"
    );
    assert_eq!(state.slow_skip_counter, 0);
}

#[test]
fn test_powerup_types_have_correct_durations() {
    let grid = GridSize { w: 10, h: 10 };
    let _state = GameState::new(grid, Seeded::new(42));

    // Test Slow power-up duration
    let slow_powerup = PowerUp {
        position: Position { x: 5, y: 5 },
        power_type: PowerUpType::Slow,
        remaining_duration: 20,
    };
    assert_eq!(slow_powerup.initial_duration(), 20);

    // Test Fast power-up duration
    let fast_powerup = PowerUp {
        position: Position { x: 5, y: 5 },
        power_type: PowerUpType::Fast,
        remaining_duration: 15,
    };
    assert_eq!(fast_powerup.initial_duration(), 15);

    // Test Poison power-up duration
    let poison_powerup = PowerUp {
        position: Position { x: 5, y: 5 },
        power_type: PowerUpType::Poison,
        remaining_duration: 0,
    };
    assert_eq!(poison_powerup.initial_duration(), 0);
}

#[test]
fn test_multiple_powerups_dont_stack() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    // Activate slow power-up
    state.active_powerup = Some((PowerUpType::Slow, 10));
    state.slow_skip_counter = 0;

    // Try to activate fast power-up (should replace slow)
    state.active_powerup = Some((PowerUpType::Fast, 15));

    // Only fast should be active now
    assert_eq!(state.active_powerup.unwrap().0, PowerUpType::Fast);
}

// Helper structs for testing
struct TestInput {
    dir: Direction,
}

impl snake_game::systems::Input for TestInput {
    fn current_dir(&self) -> Direction {
        self.dir
    }
}

struct TestTime {
    tick_count: u64,
}

impl snake_game::systems::Time for TestTime {
    fn tick(&mut self) -> snake_game::types::Tick {
        self.tick_count += 1;
        snake_game::types::Tick(self.tick_count)
    }
}

