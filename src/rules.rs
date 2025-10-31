use crate::{
    rng::RngLike,
    state::{GameState, RunState, Snake},
    types::*,
};

pub fn step<R: RngLike>(g: &mut GameState, rng: &mut R) {
    if matches!(g.run_state, RunState::Paused | RunState::Over) {
        return;
    }

    let next = next_head(g.snake.body.front().copied().unwrap(), g.snake.dir);

    // Handle wall collisions or wrapping
    #[cfg(feature = "wrap_walls")]
    let wrapped_next = if g.wrap_walls && out_of_bounds(next, g.grid) {
        wrap_position(next, g.grid)
    } else {
        next
    };

    #[cfg(not(feature = "wrap_walls"))]
    let wrapped_next = next;

    // Check for wall collisions (when wrap_walls is disabled or feature not available)
    #[cfg(feature = "wrap_walls")]
    let should_end_game = !g.wrap_walls && out_of_bounds(next, g.grid);

    #[cfg(not(feature = "wrap_walls"))]
    let should_end_game = out_of_bounds(next, g.grid);

    if should_end_game {
        g.run_state = RunState::Over;
        return;
    }

    // Check for self collisions
    if g.snake.body.iter().any(|&p| p == wrapped_next) {
        g.run_state = RunState::Over;
        return;
    }

    g.snake.body.push_front(wrapped_next);

    // Check if food is eaten (using wrapped position)
    if wrapped_next == g.food {
        g.score += 1;
        g.food = spawn_food(&g.grid, &g.snake, rng);
    } else {
        g.snake.body.pop_back();
    }
}

fn next_head(head: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => Position {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Down => Position {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Left => Position {
            x: head.x - 1,
            y: head.y,
        },
        Direction::Right => Position {
            x: head.x + 1,
            y: head.y,
        },
    }
}

fn out_of_bounds(p: Position, g: GridSize) -> bool {
    p.x < 0 || p.y < 0 || p.x >= g.w || p.y >= g.h
}

/// Wrap a position that is out of bounds to the opposite side (toroidal topology)
#[cfg(feature = "wrap_walls")]
fn wrap_position(p: Position, g: GridSize) -> Position {
    Position {
        x: p.x.rem_euclid(g.w),
        y: p.y.rem_euclid(g.h),
    }
}

fn spawn_food<R: RngLike>(grid: &GridSize, snake: &Snake, rng: &mut R) -> Position {
    // sample until empty cell found (grid small → inexpensive; tests cover termination)
    loop {
        let x = (rng.next_u32() as i32).rem_euclid(grid.w);
        let y = (rng.next_u32() as i32).rem_euclid(grid.h);
        let p = Position { x, y };
        if !snake.body.iter().any(|&s| s == p) {
            return p;
        }
    }
}
