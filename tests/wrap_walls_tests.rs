#[cfg(feature = "wrap_walls")]
use proptest::prelude::*;
#[cfg(feature = "wrap_walls")]
use snake_game::{rng::{RngLike, Seeded}, state::GameState, types::*};

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_right_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    // Position snake at right edge
    g.snake.body[0] = Position { x: 4, y: 2 };
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    // Should wrap to left edge
    assert_eq!(g.snake.body[0], Position { x: 0, y: 2 });
    assert!(!g.is_over(), "Game should continue when wrapping");
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_left_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    // Position snake at left edge
    g.snake.body[0] = Position { x: 0, y: 2 };
    g.snake.dir = Direction::Left;

    snake_game::rules::step(&mut g, &mut rng);

    // Should wrap to right edge
    assert_eq!(g.snake.body[0], Position { x: 4, y: 2 });
    assert!(!g.is_over(), "Game should continue when wrapping");
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_top_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    // Position snake at top edge
    g.snake.body[0] = Position { x: 2, y: 0 };
    g.snake.dir = Direction::Up;

    snake_game::rules::step(&mut g, &mut rng);

    // Should wrap to bottom edge
    assert_eq!(g.snake.body[0], Position { x: 2, y: 4 });
    assert!(!g.is_over(), "Game should continue when wrapping");
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_bottom_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    // Position snake at bottom edge
    g.snake.body[0] = Position { x: 2, y: 4 };
    g.snake.dir = Direction::Down;

    snake_game::rules::step(&mut g, &mut rng);

    // Should wrap to top edge
    assert_eq!(g.snake.body[0], Position { x: 2, y: 0 });
    assert!(!g.is_over(), "Game should continue when wrapping");
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_disabled_still_causes_game_over() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), false);

    // Position snake at right edge
    g.snake.body[0] = Position { x: 4, y: 2 };
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    // Should still end game when wrap_walls is false
    assert!(g.is_over(), "Game should end when wrap_walls is disabled");
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrapped_snake_never_teleports() {
    // Property: With wrap_walls enabled, consecutive head positions are always
    // Manhattan distance 1 apart (accounting for wrapping)
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(42);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    let mut prev_head = g.snake.body[0];

    // Move in different directions, including wrapping scenarios
    for dir in [
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Left,
    ] {
        g.snake.dir = dir;
        snake_game::rules::step(&mut g, &mut rng);

        if !g.is_over() {
            let new_head = g.snake.body[0];

            // Calculate Manhattan distance, accounting for wrapping
            let dx = ((new_head.x - prev_head.x).rem_euclid(grid.w)).min(
                (prev_head.x - new_head.x).rem_euclid(grid.w),
            );
            let dy = ((new_head.y - prev_head.y).rem_euclid(grid.h)).min(
                (prev_head.y - new_head.y).rem_euclid(grid.h),
            );
            let manhattan_dist = dx + dy;

            assert_eq!(
                manhattan_dist, 1,
                "Head should move exactly 1 cell (possibly wrapped), got distance {} from {:?} to {:?}",
                manhattan_dist, prev_head, new_head
            );

            prev_head = new_head;
        }
    }
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_wrap_corner_wrapping() {
    // Test wrapping from corner positions
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);

    // Top-right corner wrapping down
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);
    g.snake.body[0] = Position { x: 4, y: 0 };
    g.snake.dir = Direction::Up;
    snake_game::rules::step(&mut g, &mut rng);
    assert_eq!(g.snake.body[0], Position { x: 4, y: 4 });
    assert!(!g.is_over());

    // Bottom-left corner wrapping right
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);
    g.snake.body[0] = Position { x: 0, y: 4 };
    g.snake.dir = Direction::Left;
    snake_game::rules::step(&mut g, &mut rng);
    assert_eq!(g.snake.body[0], Position { x: 4, y: 4 });
    assert!(!g.is_over());
}

#[cfg(feature = "wrap_walls")]
#[test]
fn test_food_can_spawn_after_wrapping() {
    // Ensure food spawning still works correctly with wrap_walls enabled
    let grid = GridSize { w: 8, h: 8 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

    // Move snake around, including wrapping
    for _ in 0..100 {
        // Change direction periodically
        if rng.next_u32() % 4 == 0 {
            g.snake.dir = match rng.next_u32() % 4 {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            };
        }
        snake_game::rules::step(&mut g, &mut rng);

        if !g.is_over() {
            // Food should never spawn on snake
            #[cfg(not(feature = "multiple_foods"))]
            {
                assert!(!g.snake.body.iter().any(|&p| p == g.food));
                // Food should be within bounds
                assert!(g.food.x >= 0 && g.food.x < grid.w);
                assert!(g.food.y >= 0 && g.food.y < grid.h);
            }
            #[cfg(feature = "multiple_foods")]
            {
                for food in &g.foods {
                    assert!(!g.snake.body.iter().any(|&p| p == food.position));
                    assert!(food.position.x >= 0 && food.position.x < grid.w);
                    assert!(food.position.y >= 0 && food.position.y < grid.h);
                }
            }
        }
    }
}

#[cfg(feature = "wrap_walls")]
fn wrap_position_helper(p: Position, grid: GridSize) -> Position {
    // Helper to test wrapping logic independently
    Position {
        x: p.x.rem_euclid(grid.w),
        y: p.y.rem_euclid(grid.h),
    }
}

#[cfg(feature = "wrap_walls")]
proptest! {
    #[test]
    fn prop_wrapped_position_always_in_bounds(
        x in -100i32..200i32,
        y in -100i32..200i32,
        w in 1i32..50i32,
        h in 1i32..50i32,
    ) {
        let grid = GridSize { w, h };
        let pos = Position { x, y };
        let wrapped = wrap_position_helper(pos, grid);
        
        // Wrapped position should always be in bounds
        prop_assert!(wrapped.x >= 0 && wrapped.x < grid.w);
        prop_assert!(wrapped.y >= 0 && wrapped.y < grid.h);
    }

    #[test]
    fn prop_wrap_preserves_modulo_equivalence(
        x in -100i32..200i32,
        y in -100i32..200i32,
        w in 1i32..50i32,
        h in 1i32..50i32,
    ) {
        let grid = GridSize { w, h };
        let pos = Position { x, y };
        let wrapped = wrap_position_helper(pos, grid);
        
        // Wrapped coordinates should be equivalent modulo grid size
        prop_assert_eq!(wrapped.x.rem_euclid(grid.w), x.rem_euclid(grid.w));
        prop_assert_eq!(wrapped.y.rem_euclid(grid.h), y.rem_euclid(grid.h));
    }

    #[test]
    fn prop_wrapping_snake_never_goes_out_of_bounds(
        seed in 0u64..1000u64,
    ) {
        let grid = GridSize { w: 10, h: 10 };
        let mut rng = Seeded::new(seed);
        let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

        // Run for a reasonable number of steps
        for _ in 0..100 {
            // Randomly change direction
            if rng.next_u32() % 5 == 0 {
                g.snake.dir = match rng.next_u32() % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
            }

            if !g.is_over() {
                snake_game::rules::step(&mut g, &mut rng);

                if !g.is_over() {
                    let head_after = g.snake.body[0];
                    
                    // Head should always be in bounds when wrapping is enabled
                    prop_assert!(head_after.x >= 0 && head_after.x < grid.w);
                    prop_assert!(head_after.y >= 0 && head_after.y < grid.h);
                }
            }
        }
    }

    #[test]
    fn prop_wrapping_prevents_wall_death(
        seed in 0u64..100u64,
        w in 3i32..20i32,
        h in 3i32..20i32,
    ) {
        let grid = GridSize { w, h };
        let mut rng = Seeded::new(seed);
        let mut g = GameState::new_with_wrap(grid, rng.clone(), true);

        // Move snake directly toward a wall
        g.snake.body[0] = Position { x: grid.w - 1, y: grid.h / 2 };
        g.snake.dir = Direction::Right;

        snake_game::rules::step(&mut g, &mut rng);

        // With wrap_walls enabled, should not end game
        prop_assert!(!g.is_over(), "Wrapping should prevent wall death");
        
        // Head should be on the opposite side
        prop_assert_eq!(g.snake.body[0].x, 0);
        prop_assert_eq!(g.snake.body[0].y, grid.h / 2);
    }
}


