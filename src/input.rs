//! Input adapter: maps egui keyboard input to game Direction
//!
//! Prevents 180-degree reversals that would cause instant self-collision.

use crate::{systems::Input, types::Direction};
use eframe::egui;

/// Input adapter that tracks keyboard input from egui
#[derive(Clone)]
pub struct EguiInput {
    current_dir: Direction,
    last_settled_dir: Direction,
}

impl EguiInput {
    pub fn new(initial_dir: Direction) -> Self {
        Self {
            current_dir: initial_dir,
            last_settled_dir: initial_dir,
        }
    }

    /// Update direction based on keyboard input, preventing 180-degree reversals
    pub fn update(&mut self, ctx: &egui::Context) {
        let requested_dir = if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp))
            || ctx.input(|i| i.key_pressed(egui::Key::W))
        {
            Some(Direction::Up)
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown))
            || ctx.input(|i| i.key_pressed(egui::Key::S))
        {
            Some(Direction::Down)
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft))
            || ctx.input(|i| i.key_pressed(egui::Key::A))
        {
            Some(Direction::Left)
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight))
            || ctx.input(|i| i.key_pressed(egui::Key::D))
        {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(dir) = requested_dir {
            // Prevent 180-degree reversal
            if !self.is_opposite(dir, self.last_settled_dir) {
                self.current_dir = dir;
            }
        }
    }

    /// Mark current direction as settled (call after each game step)
    pub fn settle(&mut self) {
        self.last_settled_dir = self.current_dir;
    }

    fn is_opposite(&self, dir1: Direction, dir2: Direction) -> bool {
        matches!(
            (dir1, dir2),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

impl Input for EguiInput {
    fn current_dir(&self) -> Direction {
        self.current_dir
    }
}
