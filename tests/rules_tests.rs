use snake_game::{rng::Seeded, state::GameState, types::*};

#[test]
fn test_head_moves_in_correct_direction() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    // Set snake direction to Right
    state.snake.dir = Direction::Right;
    let original_pos = state.snake.body[0];

    snake_game::rules::step(&mut state, &mut Seeded::new(0));

    // Head should have moved right
    assert_eq!(
        state.snake.body[0],
        Position {
            x: original_pos.x + 1,
            y: original_pos.y
        }
    );
}

#[test]
fn test_tail_is_removed_after_movement() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    // Add a second segment to the snake
    let original_head = state.snake.body[0];
    let second_pos = Position {
        x: original_head.x - 1,
        y: original_head.y,
    };
    state.snake.body.push_back(second_pos);

    let original_len = state.snake.body.len();
    assert_eq!(original_len, 2); // Should be 2 now

    snake_game::rules::step(&mut state, &mut Seeded::new(0));

    // Length should remain the same (head added, tail removed)
    assert_eq!(state.snake.body.len(), original_len);

    // After movement, the snake should be: [new_head, original_head]
    // So the last element should be the original head position
    assert_eq!(state.snake.body[1], original_head);
}

#[test]
fn test_movement_with_different_directions() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    let initial_pos = state.snake.body[0];

    // Test moving Up
    state.snake.dir = Direction::Up;
    snake_game::rules::step(&mut state, &mut Seeded::new(0));
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_pos.x,
            y: initial_pos.y - 1
        }
    );

    // Test moving Down
    state.snake.dir = Direction::Down;
    snake_game::rules::step(&mut state, &mut Seeded::new(0));
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_pos.x,
            y: initial_pos.y
        }
    );

    // Test moving Left
    state.snake.dir = Direction::Left;
    snake_game::rules::step(&mut state, &mut Seeded::new(0));
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_pos.x - 1,
            y: initial_pos.y
        }
    );

    // Test moving Right
    state.snake.dir = Direction::Right;
    snake_game::rules::step(&mut state, &mut Seeded::new(0));
    assert_eq!(
        state.snake.body[0],
        Position {
            x: initial_pos.x,
            y: initial_pos.y
        }
    );
}

#[test]
fn test_snake_body_maintains_correct_sequence() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    // Create a 3-segment snake in a line
    let pos2 = Position {
        x: state.snake.body[0].x - 1,
        y: state.snake.body[0].y,
    };
    let pos3 = Position {
        x: state.snake.body[0].x - 2,
        y: state.snake.body[0].y,
    };
    state.snake.body.push_back(pos2);
    state.snake.body.push_back(pos3);

    // Remember original positions
    let original_head = state.snake.body[0];
    let original_second = state.snake.body[1];
    let _original_third = state.snake.body[2];

    // Move right (extending the line)
    state.snake.dir = Direction::Right;
    snake_game::rules::step(&mut state, &mut Seeded::new(0));

    // The new head should be one position right of the original head
    assert_eq!(
        state.snake.body[0],
        Position {
            x: original_head.x + 1,
            y: original_head.y
        }
    );

    // The second segment should be where the original head was
    assert_eq!(state.snake.body[1], original_head);

    // The third segment should be where the original second was
    assert_eq!(state.snake.body[2], original_second);
}

#[test]
fn test_game_over_state_not_affected_by_movement() {
    let grid = GridSize { w: 10, h: 10 };
    let mut state = GameState::new(grid, Seeded::new(42));

    // Initially game should not be over
    assert!(!state.is_over());

    snake_game::rules::step(&mut state, &mut Seeded::new(0));

    // Game should still not be over after movement
    assert!(!state.is_over());
}

#[test]
#[cfg(not(feature = "multiple_foods"))]
fn food_never_spawns_on_snake() {
    let grid = GridSize { w: 8, h: 8 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());
    for _ in 0..1000 {
        snake_game::rules::step(&mut g, &mut rng);
        assert!(!g.snake.body.iter().any(|&p| p == g.food));
    }
}

#[test]
fn hits_wall_is_game_over() {
    let grid = GridSize { w: 3, h: 3 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());
    g.snake.dir = Direction::Left; // move into wall quickly
    for _ in 0..3 {
        snake_game::rules::step(&mut g, &mut rng);
    }
    assert!(g.is_over());
}

#[test]
fn test_wall_collision_top_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());
    // Position snake at top edge
    g.snake.body[0] = Position { x: 2, y: 0 };
    g.snake.dir = Direction::Up;
    snake_game::rules::step(&mut g, &mut rng);
    assert!(g.is_over(), "Should hit top wall");
}

#[test]
fn test_wall_collision_bottom_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());
    // Position snake at bottom edge
    g.snake.body[0] = Position { x: 2, y: 4 };
    g.snake.dir = Direction::Down;
    snake_game::rules::step(&mut g, &mut rng);
    assert!(g.is_over(), "Should hit bottom wall");
}

#[test]
fn test_wall_collision_left_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());
    // Position snake at left edge
    g.snake.body[0] = Position { x: 0, y: 2 };
    g.snake.dir = Direction::Left;
    snake_game::rules::step(&mut g, &mut rng);
    assert!(g.is_over(), "Should hit left wall");
}

#[test]
fn test_wall_collision_right_edge() {
    let grid = GridSize { w: 5, h: 5 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());
    // Position snake at right edge
    g.snake.body[0] = Position { x: 4, y: 2 };
    g.snake.dir = Direction::Right;
    snake_game::rules::step(&mut g, &mut rng);
    assert!(g.is_over(), "Should hit right wall");
}

#[test]
fn test_self_collision_loop_scenario() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(1);
    let mut g = GameState::new(grid, rng.clone());

    // Create a snake that forms a square loop
    // Head is at (3,4), body goes: (3,4) -> (4,4) -> (4,3) -> (3,3)
    // Moving Up from (3,4) will hit (3,3) which is in the body
    g.snake.body.clear();
    g.snake.body.push_front(Position { x: 3, y: 4 }); // Head
    g.snake.body.push_back(Position { x: 4, y: 4 });
    g.snake.body.push_back(Position { x: 4, y: 3 });
    g.snake.body.push_back(Position { x: 3, y: 3 }); // Tail

    // Set direction to Up, which will try to move to (3,3) and collide
    g.snake.dir = Direction::Up;
    snake_game::rules::step(&mut g, &mut rng);
    assert!(g.is_over(), "Should detect self-collision in loop");
}

#[test]
fn test_no_teleport_movement() {
    // Verify that consecutive head positions are always Manhattan distance 1 apart
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(42);
    let mut g = GameState::new(grid, rng.clone());

    let mut prev_head = g.snake.body[0];

    // Move in different directions
    for dir in [
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ] {
        g.snake.dir = dir;
        snake_game::rules::step(&mut g, &mut rng);

        if !g.is_over() {
            let new_head = g.snake.body[0];
            let dx = (new_head.x - prev_head.x).abs();
            let dy = (new_head.y - prev_head.y).abs();
            let manhattan_dist = dx + dy;

            assert_eq!(
                manhattan_dist, 1,
                "Head should move exactly 1 cell, got distance {}",
                manhattan_dist
            );

            prev_head = new_head;
        }
    }
}

#[test]
#[cfg(not(feature = "multiple_foods"))]
fn test_eating_food_increments_score() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    let initial_score = g.score;

    // Position snake head next to food
    let food_pos = g.food;
    g.snake.body[0] = Position {
        x: food_pos.x - 1,
        y: food_pos.y,
    };
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    assert_eq!(g.score, initial_score + 1, "Score should increment");
    assert_ne!(g.food, food_pos, "Food should respawn");
    assert!(
        !g.snake.body.iter().any(|&p| p == g.food),
        "New food should not spawn on snake"
    );
}
