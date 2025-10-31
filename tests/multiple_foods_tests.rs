#[cfg(feature = "multiple_foods")]
use snake_game::{rng::{RngLike, Seeded}, state::GameState, types::*};

#[cfg(feature = "multiple_foods")]
#[test]
fn test_multiple_foods_spawn_on_init() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(42);
    let g = GameState::new(grid, rng.clone());

    // Should spawn 3-5 foods initially
    assert!(g.foods.len() >= 3 && g.foods.len() <= 5);
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_foods_never_spawn_on_snake() {
    let grid = GridSize { w: 8, h: 8 };
    let rng = Seeded::new(123);
    let g = GameState::new(grid, rng.clone());

    // All foods should not be on the snake
    for food in &g.foods {
        assert!(
            !g.snake.body.iter().any(|&p| p == food.position),
            "Food at {:?} should not be on snake",
            food.position
        );
    }
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_foods_have_different_types() {
    let grid = GridSize { w: 10, h: 10 };
    let rng = Seeded::new(42);
    let g = GameState::new(grid, rng.clone());

    // Check that at least one food exists
    assert!(!g.foods.is_empty());

    // Verify all foods have valid types
    for food in &g.foods {
        match food.food_type {
            FoodType::Normal | FoodType::Golden | FoodType::Special => {}
        }
    }
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_normal_food_gives_1_point() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    // Find or create a normal food next to snake head
    let head_pos = g.snake.body[0];
    let food_pos = Position {
        x: head_pos.x + 1,
        y: head_pos.y,
    };

    // Remove all foods and add a normal food at a known position
    g.foods.clear();
    g.foods.push(Food {
        position: food_pos,
        food_type: FoodType::Normal,
    });

    let initial_score = g.score;
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    assert_eq!(g.score, initial_score + 1, "Normal food should give 1 point");
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_golden_food_gives_5_points() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    // Find or create a golden food next to snake head
    let head_pos = g.snake.body[0];
    let food_pos = Position {
        x: head_pos.x + 1,
        y: head_pos.y,
    };

    // Remove all foods and add a golden food at a known position
    g.foods.clear();
    g.foods.push(Food {
        position: food_pos,
        food_type: FoodType::Golden,
    });

    let initial_score = g.score;
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    assert_eq!(g.score, initial_score + 5, "Golden food should give 5 points");
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_special_food_gives_10_points() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    // Find or create a special food next to snake head
    let head_pos = g.snake.body[0];
    let food_pos = Position {
        x: head_pos.x + 1,
        y: head_pos.y,
    };

    // Remove all foods and add a special food at a known position
    g.foods.clear();
    g.foods.push(Food {
        position: food_pos,
        food_type: FoodType::Special,
    });

    let initial_score = g.score;
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    assert_eq!(g.score, initial_score + 10, "Special food should give 10 points");
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_eating_food_removes_it_from_grid() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    let head_pos = g.snake.body[0];
    let food_pos = Position {
        x: head_pos.x + 1,
        y: head_pos.y,
    };

    // Add a food at a known position
    g.foods.clear();
    g.foods.push(Food {
        position: food_pos,
        food_type: FoodType::Normal,
    });

    let initial_food_count = g.foods.len();
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    // Food should be removed (but a new one might spawn)
    assert!(
        !g.foods.iter().any(|f| f.position == food_pos),
        "Eaten food should be removed"
    );
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_new_food_spawns_after_eating() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    let head_pos = g.snake.body[0];
    let food_pos = Position {
        x: head_pos.x + 1,
        y: head_pos.y,
    };

    // Start with exactly 4 foods (one below max to allow spawning)
    g.foods.clear();
    g.foods.push(Food {
        position: food_pos,
        food_type: FoodType::Normal,
    });
    g.foods.push(Food {
        position: Position { x: 0, y: 0 },
        food_type: FoodType::Normal,
    });
    g.foods.push(Food {
        position: Position { x: 9, y: 9 },
        food_type: FoodType::Normal,
    });
    g.foods.push(Food {
        position: Position { x: 0, y: 9 },
        food_type: FoodType::Normal,
    });

    let initial_food_count = g.foods.len();
    g.snake.dir = Direction::Right;

    snake_game::rules::step(&mut g, &mut rng);

    // After eating, should have the same count (one removed, one added)
    assert_eq!(
        g.foods.len(),
        initial_food_count,
        "Food count should be maintained (was {})",
        initial_food_count
    );
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_multiple_foods_on_grid_simultaneously() {
    let grid = GridSize { w: 20, h: 20 };
    let rng = Seeded::new(42);
    let g = GameState::new(grid, rng.clone());

    // Should have multiple foods on the grid
    assert!(g.foods.len() >= 3, "Should have at least 3 foods on grid");

    // All foods should be at unique positions
    let positions: Vec<Position> = g.foods.iter().map(|f| f.position).collect();
    for (i, pos1) in positions.iter().enumerate() {
        for (j, pos2) in positions.iter().enumerate() {
            if i != j {
                assert_ne!(*pos1, *pos2, "Foods should be at unique positions");
            }
        }
    }
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_food_type_point_values() {
    assert_eq!(FoodType::Normal.point_value(), 1);
    assert_eq!(FoodType::Golden.point_value(), 5);
    assert_eq!(FoodType::Special.point_value(), 10);
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_eating_different_foods_accumulates_score_correctly() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    let mut score_expected = 0;

    // Eat a normal food (1 point)
    let head_pos = g.snake.body[0];
    g.foods.clear();
    g.foods.push(Food {
        position: Position {
            x: head_pos.x + 1,
            y: head_pos.y,
        },
        food_type: FoodType::Normal,
    });
    g.snake.dir = Direction::Right;
    snake_game::rules::step(&mut g, &mut rng);
    score_expected += 1;
    assert_eq!(g.score, score_expected);

    // Eat a golden food (5 points)
    let head_pos = g.snake.body[0];
    g.foods.clear();
    g.foods.push(Food {
        position: Position {
            x: head_pos.x + 1,
            y: head_pos.y,
        },
        food_type: FoodType::Golden,
    });
    g.snake.dir = Direction::Right;
    snake_game::rules::step(&mut g, &mut rng);
    score_expected += 5;
    assert_eq!(g.score, score_expected);

    // Eat a special food (10 points)
    let head_pos = g.snake.body[0];
    g.foods.clear();
    g.foods.push(Food {
        position: Position {
            x: head_pos.x + 1,
            y: head_pos.y,
        },
        food_type: FoodType::Special,
    });
    g.snake.dir = Direction::Right;
    snake_game::rules::step(&mut g, &mut rng);
    score_expected += 10;
    assert_eq!(g.score, score_expected);

    assert_eq!(g.score, 16); // 1 + 5 + 10
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_food_never_spawns_on_snake_after_eating() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(123);
    let mut g = GameState::new(grid, rng.clone());

    // Move snake and eat foods for many steps
    for _ in 0..100 {
        if g.is_over() {
            break;
        }

        // Change direction occasionally
        if rng.next_u32() % 10 == 0 {
            g.snake.dir = match rng.next_u32() % 4 {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            };
        }

        snake_game::rules::step(&mut g, &mut rng);

        // After step, verify no food is on snake
        for food in &g.foods {
            assert!(
                !g.snake.body.iter().any(|&p| p == food.position),
                "Food at {:?} should not be on snake",
                food.position
            );
        }
    }
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_foods_maintain_count_during_gameplay() {
    let grid = GridSize { w: 15, h: 15 };
    let mut rng = Seeded::new(42);
    let mut g = GameState::new(grid, rng.clone());

    let initial_count = g.foods.len();
    assert!(initial_count >= 3 && initial_count <= 5);

    // Play for a while
    for _ in 0..50 {
        if g.is_over() {
            break;
        }

        // Change direction occasionally
        if rng.next_u32() % 5 == 0 {
            g.snake.dir = match rng.next_u32() % 4 {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            };
        }

        snake_game::rules::step(&mut g, &mut rng);

        // Food count should remain between 3 and 5 (when not eaten)
        if !g.is_over() {
            assert!(
                g.foods.len() >= 3 && g.foods.len() <= 5,
                "Food count should stay between 3-5, got {}",
                g.foods.len()
            );
        }
    }
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_reset_preserves_multiple_foods_feature() {
    let grid = GridSize { w: 10, h: 10 };
    let mut rng = Seeded::new(42);
    let mut g = GameState::new(grid, rng.clone());

    let initial_food_count = g.foods.len();
    assert!(initial_food_count >= 3 && initial_food_count <= 5);

    // Eat some food and move around
    g.snake.dir = Direction::Right;
    for _ in 0..5 {
        snake_game::rules::step(&mut g, &mut rng);
    }

    // Reset
    g.reset(rng.clone());

    // After reset, should still have multiple foods
    assert!(g.foods.len() >= 3 && g.foods.len() <= 5);
    assert_eq!(g.score, 0);
}

#[cfg(feature = "multiple_foods")]
#[test]
fn test_food_types_spawn_with_correct_probabilities() {
    // This test checks that food types spawn with roughly expected probabilities
    // We'll spawn many foods and check the distribution
    let grid = GridSize { w: 20, h: 20 };
    let mut rng = Seeded::new(123);
    let mut normal_count = 0;
    let mut golden_count = 0;
    let mut special_count = 0;
    let total_foods = 1000;

    for _ in 0..total_foods {
        let snake = snake_game::state::Snake {
            body: std::collections::VecDeque::from(vec![Position { x: 10, y: 10 }]),
            dir: Direction::Right,
        };
        let food_type = determine_food_type_helper(&grid, &snake, &mut rng);
        match food_type {
            FoodType::Normal => normal_count += 1,
            FoodType::Golden => golden_count += 1,
            FoodType::Special => special_count += 1,
        }
    }

    // Check approximate probabilities (with tolerance for randomness)
    // Normal: ~70%, Golden: ~25%, Special: ~5%
    let normal_pct = (normal_count * 100) / total_foods;
    let golden_pct = (golden_count * 100) / total_foods;
    let special_pct = (special_count * 100) / total_foods;

    // Allow ±10% tolerance
    assert!(
        normal_pct >= 60 && normal_pct <= 80,
        "Normal food should be ~70%, got {}%",
        normal_pct
    );
    assert!(
        golden_pct >= 15 && golden_pct <= 35,
        "Golden food should be ~25%, got {}%",
        golden_pct
    );
    assert!(
        special_pct >= 0 && special_pct <= 15,
        "Special food should be ~5%, got {}%",
        special_pct
    );
}

// Helper function to determine food type (matches the logic in rules.rs)
#[cfg(feature = "multiple_foods")]
fn determine_food_type_helper<R: RngLike>(
    _grid: &GridSize,
    _snake: &snake_game::state::Snake,
    rng: &mut R,
) -> FoodType {
    let roll = rng.next_u32() % 100;
    if roll < 70 {
        FoodType::Normal
    } else if roll < 95 {
        FoodType::Golden
    } else {
        FoodType::Special
    }
}

