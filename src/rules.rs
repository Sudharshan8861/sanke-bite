use crate::{
    rng::RngLike,
    state::{GameState, RunState, Snake},
    types::*,
};
#[cfg(feature = "multiple_foods")]
use crate::types::{Food, FoodType};
#[cfg(feature = "powerups")]
use crate::types::{PowerUp, PowerUpType};

pub fn step<R: RngLike>(g: &mut GameState, rng: &mut R) {
    if matches!(g.run_state, RunState::Paused | RunState::Over) {
        return;
    }

    // Note: Duration tracking for power-ups is handled in the game loop (systems.rs)
    // not here, so it decrements once per update cycle, not per step

    // Handle slow power-up: skip every other step
    #[cfg(feature = "powerups")]
    {
        if let Some((PowerUpType::Slow, _)) = g.active_powerup {
            g.slow_skip_counter += 1;
            if g.slow_skip_counter % 2 == 1 {
                return; // Skip this step
            }
        }
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
    #[cfg(not(feature = "multiple_foods"))]
    {
        if wrapped_next == g.food {
            g.score += 1;
            g.food = spawn_food(&g.grid, &g.snake, rng);
        } else {
            g.snake.body.pop_back();
        }
    }

    #[cfg(feature = "multiple_foods")]
    {
        let mut food_eaten = false;
        
        // Check if snake head collides with any food
        if let Some(food_index) = g.foods.iter().position(|f| f.position == wrapped_next) {
            let eaten_food = g.foods.remove(food_index);
            let points_earned = eaten_food.food_type.point_value();
            g.score += points_earned;
            food_eaten = true;
            
            // Spawn a new food to maintain food count (keep 3-5 foods on grid)
            if g.foods.len() < 5 {
                let new_food = spawn_food_with_type(&g.grid, &g.snake, rng, &g.foods);
                g.foods.push(new_food);
            }
        }
        
        if !food_eaten {
            g.snake.body.pop_back();
        }
    }

    // Check for power-up collision
    #[cfg(feature = "powerups")]
    {
        if let Some(powerup) = g.powerup {
            if wrapped_next == powerup.position {
                let power_type = powerup.power_type;
                let duration = powerup.initial_duration();
                
                // Apply immediate effects (poison)
                match power_type {
                    PowerUpType::Poison => {
                        // Shrink snake by removing tail segment
                        if g.snake.body.len() > 1 {
                            g.snake.body.pop_back();
                        }
                        // Poison has no duration, so don't set active power-up
                    }
                    _ => {
                        // Activate power-up for slow/fast (they have duration)
                        g.active_powerup = Some((power_type, duration));
                        // Reset slow skip counter when activating slow
                        if matches!(power_type, PowerUpType::Slow) {
                            g.slow_skip_counter = 0;
                        }
                    }
                }
                
                // Remove power-up from grid
                g.powerup = None;
            }
        }
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

#[cfg(feature = "powerups")]
fn spawn_powerup<R: RngLike>(
    grid: &GridSize,
    snake: &Snake,
    rng: &mut R,
    existing_powerup: &Option<PowerUp>,
    #[cfg(feature = "multiple_foods")]
    existing_foods: &[Food],
    #[cfg(not(feature = "multiple_foods"))]
    food: Position,
) -> Option<PowerUp> {
    // Don't spawn if one already exists
    if existing_powerup.is_some() {
        return None;
    }

    // Spawn with 10% chance per step (roughly every 10 steps)
    if (rng.next_u32() % 100) >= 10 {
        return None;
    }

    let power_type = determine_powerup_type(rng);
    
    let mut attempts = 0;
    loop {
        if attempts > 100 {
            // Give up after 100 attempts
            return None;
        }
        attempts += 1;

        let x = (rng.next_u32() as i32).rem_euclid(grid.w);
        let y = (rng.next_u32() as i32).rem_euclid(grid.h);
        let p = Position { x, y };

        // Check not on snake
        if snake.body.iter().any(|&s| s == p) {
            continue;
        }

        // Check not on food
        #[cfg(not(feature = "multiple_foods"))]
        if p == food {
            continue;
        }

        #[cfg(feature = "multiple_foods")]
        if existing_foods.iter().any(|f| f.position == p) {
            continue;
        }

        // Check not on existing power-up
        if let Some(existing) = existing_powerup {
            if existing.position == p {
                continue;
            }
        }

        let duration = match power_type {
            PowerUpType::Slow => 20,
            PowerUpType::Fast => 15,
            PowerUpType::Poison => 0,
        };

        return Some(PowerUp {
            position: p,
            power_type,
            remaining_duration: duration,
        });
    }
}

#[cfg(feature = "powerups")]
fn determine_powerup_type<R: RngLike>(rng: &mut R) -> PowerUpType {
    // Spawn probabilities:
    // Slow: 40% (0-39)
    // Fast: 40% (40-79)
    // Poison: 20% (80-99)
    let roll = rng.next_u32() % 100;
    if roll < 40 {
        PowerUpType::Slow
    } else if roll < 80 {
        PowerUpType::Fast
    } else {
        PowerUpType::Poison
    }
}


#[cfg(feature = "powerups")]
pub fn try_spawn_powerup<R: RngLike>(g: &mut GameState, rng: &mut R) {
    if g.powerup.is_some() {
        return; // Already has a power-up on the grid
    }

    #[cfg(not(feature = "multiple_foods"))]
    {
        if let Some(new_powerup) = spawn_powerup(&g.grid, &g.snake, rng, &g.powerup, g.food) {
            g.powerup = Some(new_powerup);
        }
    }

    #[cfg(feature = "multiple_foods")]
    {
        if let Some(new_powerup) = spawn_powerup(&g.grid, &g.snake, rng, &g.powerup, &g.foods) {
            g.powerup = Some(new_powerup);
        }
    }
}
