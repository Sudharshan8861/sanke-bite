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
        crate::rules::step(g, &mut self.rng);
        let _ = self.time.tick();
    }
}
