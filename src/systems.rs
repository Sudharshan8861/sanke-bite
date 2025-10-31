//! Ports & Adapters pattern: Input and Time traits, plus the game loop
//!
//! Frontends implement `Input` and `Time` traits to drive the game loop.
//! Tests can plug in mocks for deterministic sequences.

use crate::{rng::RngLike, state::GameState, types::*};

/// Input port: provides the current desired direction for the snake
pub trait Input {
    fn current_dir(&self) -> Direction;
}

/// Time port: tracks game ticks
pub trait Time {
    fn tick(&mut self) -> Tick;
}

/// Game loop that combines Input, Time, and RNG to update game state
pub struct Loop<S: Input, T: Time, R: RngLike> {
    pub input: S,
    pub time: T,
    pub rng: R,
}

impl<S: Input, T: Time, R: RngLike> Loop<S, T, R> {
    /// Update the game state based on current input, time, and RNG
    pub fn update(&mut self, g: &mut GameState) {
        g.snake.dir = self.input.current_dir();
        
        // Update power-up duration (once per update cycle)
        #[cfg(feature = "powerups")]
        {
            update_powerup_effects(g);
        }
        
        // Try to spawn power-ups
        #[cfg(feature = "powerups")]
        {
            crate::rules::try_spawn_powerup(g, &mut self.rng);
        }
        
        // Handle fast power-up: move twice per update
        #[cfg(feature = "powerups")]
        {
            if let Some((crate::types::PowerUpType::Fast, _)) = g.active_powerup {
                // Fast mode: execute step twice
                crate::rules::step(g, &mut self.rng);
                crate::rules::step(g, &mut self.rng);
            } else {
                crate::rules::step(g, &mut self.rng);
            }
        }
        
        #[cfg(not(feature = "powerups"))]
        {
            crate::rules::step(g, &mut self.rng);
        }
        
        let _ = self.time.tick();
    }
}

#[cfg(feature = "powerups")]
fn update_powerup_effects(g: &mut GameState) {
    // Update active power-up duration (once per update cycle)
    if let Some((_power_type, ref mut duration)) = g.active_powerup {
        if *duration > 0 {
            *duration -= 1;
        }
        
        // Remove power-up when duration expires
        if *duration == 0 {
            g.active_powerup = None;
            g.slow_skip_counter = 0;
        }
    }
}
