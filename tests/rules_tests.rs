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

