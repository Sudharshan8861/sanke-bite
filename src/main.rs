mod input;
mod render;
mod rng;
mod rules;
mod state;
mod systems;
mod types;

use eframe::egui;
use systems::{Loop, Time};
use types::Tick;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Snake Game",
        native_options,
        Box::new(|_cc| Box::new(SnakeApp::default())),
    )
    .expect("Failed to run application");
}

/// Simple Time implementation for egui app
struct EguiTime {
    current_tick: u64,
}

impl EguiTime {
    fn new() -> Self {
        Self { current_tick: 0 }
    }
}

impl Time for EguiTime {
    fn tick(&mut self) -> Tick {
        self.current_tick += 1;
        Tick(self.current_tick)
    }
}

struct SnakeApp {
    game_state: state::GameState,
    input: input::EguiInput,
    loop_system: Loop<input::EguiInput, EguiTime, rng::Seeded>,
    last_update_time: std::time::Instant,
    update_interval: std::time::Duration,
}

impl Default for SnakeApp {
    fn default() -> Self {
        let grid = types::GridSize { w: 20, h: 20 };
        let rng = rng::Seeded::new(42);
        let game_state = state::GameState::new(grid, rng.clone());

        let initial_dir = game_state.snake.dir;
        let input = input::EguiInput::new(initial_dir);
        let time = EguiTime::new();

        let loop_system = Loop {
            input: input.clone(),
            time,
            rng,
        };

        Self {
            game_state,
            input,
            loop_system,
            last_update_time: std::time::Instant::now(),
            update_interval: std::time::Duration::from_millis(150), // ~6.67 fps
        }
    }
}

impl eframe::App for SnakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update input
        self.input.update(ctx);

        // Handle pause toggle
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            if self.game_state.is_paused() {
                self.game_state.resume();
            } else {
                self.game_state.pause();
            }
        }

        // Reset game
        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            let rng = self.loop_system.rng.clone();
            self.game_state.reset(rng.clone());
            self.input = input::EguiInput::new(types::Direction::Right);
            self.loop_system.rng = rng;
            self.loop_system.input = self.input.clone();
        }

        // Update game loop at fixed interval
        if !self.game_state.is_paused() && !self.game_state.is_over() {
            let now = std::time::Instant::now();
            if now.duration_since(self.last_update_time) >= self.update_interval {
                self.loop_system.input = self.input.clone();
                self.loop_system.update(&mut self.game_state);
                self.input.settle();
                self.last_update_time = now;
            }
        }

        // Render
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.max_rect();
            let painter = ui.painter();

            render::render_game(painter, available_rect, &self.game_state);

            // Show controls
            ui.allocate_space(egui::vec2(0.0, available_rect.height() - 100.0));
            ui.horizontal(|ui| {
                ui.label("Controls: Arrow Keys/WASD - Move | Space - Pause | R - Reset");
            });
        });

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
}
