use crate::{rng::RngLike, types::*};
#[cfg(feature = "multiple_foods")]
use crate::types::{Food, FoodType};
#[cfg(feature = "powerups")]
use crate::types::{PowerUp, PowerUpType};
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
    #[cfg(not(feature = "multiple_foods"))]
    pub food: Position,
    #[cfg(feature = "multiple_foods")]
    pub foods: Vec<Food>,
    pub score: u32,
    pub run_state: RunState,
    #[cfg(feature = "wrap_walls")]
    pub wrap_walls: bool,
    #[cfg(feature = "powerups")]
    pub powerup: Option<PowerUp>,
    #[cfg(feature = "powerups")]
    pub active_powerup: Option<(PowerUpType, u32)>, // (type, remaining_duration)
    #[cfg(feature = "powerups")]
    pub slow_skip_counter: u32, // Skip every other step when slow is active
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

    #[cfg(all(feature = "wrap_walls", not(feature = "multiple_foods"), not(feature = "powerups")))]
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

    #[cfg(all(feature = "wrap_walls", not(feature = "multiple_foods"), feature = "powerups"))]
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
            powerup: None,
            active_powerup: None,
            slow_skip_counter: 0,
        }
    }

    #[cfg(all(feature = "wrap_walls", feature = "multiple_foods", not(feature = "powerups")))]
    pub fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let foods = spawn_initial_foods(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            foods,
            score: 0,
            run_state: RunState::Running,
            wrap_walls,
        }
    }

    #[cfg(all(feature = "wrap_walls", feature = "multiple_foods", feature = "powerups"))]
    pub fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let foods = spawn_initial_foods(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            foods,
            score: 0,
            run_state: RunState::Running,
            wrap_walls,
            powerup: None,
            active_powerup: None,
            slow_skip_counter: 0,
        }
    }

    #[cfg(all(not(feature = "wrap_walls"), not(feature = "multiple_foods"), not(feature = "powerups")))]
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

    #[cfg(all(not(feature = "wrap_walls"), not(feature = "multiple_foods"), feature = "powerups"))]
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
            powerup: None,
            active_powerup: None,
            slow_skip_counter: 0,
        }
    }

    #[cfg(all(not(feature = "wrap_walls"), feature = "multiple_foods", not(feature = "powerups")))]
    fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, _wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let foods = spawn_initial_foods(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            foods,
            score: 0,
            run_state: RunState::Running,
        }
    }

    #[cfg(all(not(feature = "wrap_walls"), feature = "multiple_foods", feature = "powerups"))]
    fn new_with_wrap<R: RngLike>(grid: GridSize, mut rng: R, _wrap_walls: bool) -> Self {
        let start = Position {
            x: grid.w / 2,
            y: grid.h / 2,
        };

        let snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };

        let foods = spawn_initial_foods(&grid, &snake, &mut rng);

        Self {
            grid,
            snake,
            foods,
            score: 0,
            run_state: RunState::Running,
            powerup: None,
            active_powerup: None,
            slow_skip_counter: 0,
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

    #[cfg(all(not(feature = "multiple_foods"), not(feature = "powerups")))]
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

    #[cfg(all(not(feature = "multiple_foods"), feature = "powerups"))]
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
        self.powerup = None;
        self.active_powerup = None;
        self.slow_skip_counter = 0;
        // wrap_walls setting is preserved on reset
    }

    #[cfg(all(feature = "multiple_foods", not(feature = "powerups")))]
    pub fn reset<R: RngLike>(&mut self, mut rng: R) {
        let start = Position {
            x: self.grid.w / 2,
            y: self.grid.h / 2,
        };

        self.snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };
        self.foods = spawn_initial_foods(&self.grid, &self.snake, &mut rng);
        self.score = 0;
        self.run_state = RunState::Running;
        // wrap_walls setting is preserved on reset
    }

    #[cfg(all(feature = "multiple_foods", feature = "powerups"))]
    pub fn reset<R: RngLike>(&mut self, mut rng: R) {
        let start = Position {
            x: self.grid.w / 2,
            y: self.grid.h / 2,
        };

        self.snake = Snake {
            body: std::iter::once(start).collect(),
            dir: Direction::Right,
        };
        self.foods = spawn_initial_foods(&self.grid, &self.snake, &mut rng);
        self.score = 0;
        self.run_state = RunState::Running;
        self.powerup = None;
        self.active_powerup = None;
        self.slow_skip_counter = 0;
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

#[cfg(feature = "multiple_foods")]
fn spawn_initial_foods<R: RngLike>(grid: &GridSize, snake: &Snake, rng: &mut R) -> Vec<Food> {
    let mut foods = Vec::new();
    
    // Spawn 3-5 foods initially, with a mix of types
    let num_foods = 3 + ((rng.next_u32() % 3) as usize); // 3-5 foods
    
    for _ in 0..num_foods {
        let food = spawn_food_with_type(grid, snake, rng, &foods);
        foods.push(food);
    }
    
    foods
}

#[cfg(feature = "multiple_foods")]
fn spawn_food_with_type<R: RngLike>(
    grid: &GridSize,
    snake: &Snake,
    rng: &mut R,
    existing_foods: &[Food],
) -> Food {
    let food_type = determine_food_type(rng);
    
    loop {
        let x = (rng.next_u32() as i32).rem_euclid(grid.w);
        let y = (rng.next_u32() as i32).rem_euclid(grid.h);
        let p = Position { x, y };

        // Check not on snake and not on existing foods
        if !snake.body.iter().any(|&s| s == p)
            && !existing_foods.iter().any(|f| f.position == p)
        {
            return Food {
                position: p,
                food_type,
            };
        }
    }
}

#[cfg(feature = "multiple_foods")]
fn determine_food_type<R: RngLike>(rng: &mut R) -> FoodType {
    // Spawn probabilities:
    // Normal: 70% (0-69)
    // Golden: 25% (70-94)
    // Special: 5% (95-99)
    let roll = rng.next_u32() % 100;
    if roll < 70 {
        FoodType::Normal
    } else if roll < 95 {
        FoodType::Golden
    } else {
        FoodType::Special
    }
}
