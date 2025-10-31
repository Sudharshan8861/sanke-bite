use crate::{rng::RngLike, types::*};
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snake {
    pub body: VecDeque<Position>,
    pub dir: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunState {
    Running,
    Paused,
    Over,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub grid: GridSize,
    pub snake: Snake,
    pub food: Position,
    pub score: u32,
    pub run_state: RunState,
    #[cfg(feature = "wrap_walls")]
    pub wrap_walls: bool,
}

impl GameState {
    #[cfg(not(feature = "wrap_walls"))]
    pub fn new<R: RngLike>(grid: GridSize, rng: R) -> Self {
        Self::new_with_wrap(grid, rng, false)
    }

    #[cfg(feature = "wrap_walls")]
    pub fn new<R: RngLike>(grid: GridSize, rng: R) -> Self {
        Self::new_with_wrap(grid, rng, false)
    }

    #[cfg(feature = "wrap_walls")]
    pub fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let food = spawn_food(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            food,
            score: 0,
            run_state: RunState::Running,
            wrap_walls,
        }
    }

    #[cfg(not(feature = "wrap_walls"))]
    fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, _wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let food = spawn_food(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            food,
            score: 0,
            run_state: RunState::Running,
        }
    }

    pub fn pause(&mut self) {
        if matches!(self.run_state, RunState::Running) {
            self.run_state = RunState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if matches!(self.run_state, RunState::Paused) {
            self.run_state = RunState::Running;
        }
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.run_state, RunState::Paused)
    }

    pub fn is_over(&self) -> bool {
        matches!(self.run_state, RunState::Over)
    }

    pub fn reset<R: RngLike>(&mut self, mut rng: R) {
        let start = Position {
            x: self.grid.w / 2,
            y: self.grid.h / 2,
        };

        self.snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };
        self.food = spawn_food(&self.grid, &self.snake, &mut rng);
        self.score = 0;
        self.run_state = RunState::Running;
        // wrap_walls setting is preserved on reset
    }
}

fn spawn_food<R: RngLike>(grid: &GridSize, snake: &Snake, rng: &mut R) -> Position {
    loop {
        let x = (rng.next_u32() as i32).rem_euclid(grid.w);
        let y = (rng.next_u32() as i32).rem_euclid(grid.h);
        let p = Position { x, y };

        if !snake.body.iter().any(|&s| s == p) {
            return p;
        }
    }
}
