//! Rendering module: draws grid, snake, food, and HUD using egui::Painter

use crate::{state::GameState, types::*};
#[cfg(feature = "multiple_foods")]
use crate::types::{Food, FoodType};
use eframe::egui::{self, Color32, Painter, Rect, Stroke, Style, TextStyle};

const CELL_MARGIN: f32 = 1.0;
const GRID_COLOR: Color32 = Color32::from_rgb(40, 40, 40);
const SNAKE_COLOR: Color32 = Color32::from_rgb(0, 200, 0);
const FOOD_COLOR: Color32 = Color32::from_rgb(200, 0, 0);
const HEAD_COLOR: Color32 = Color32::from_rgb(0, 255, 0);

#[cfg(feature = "multiple_foods")]
const NORMAL_FOOD_COLOR: Color32 = Color32::from_rgb(200, 0, 0);
#[cfg(feature = "multiple_foods")]
const GOLDEN_FOOD_COLOR: Color32 = Color32::from_rgb(255, 215, 0);
#[cfg(feature = "multiple_foods")]
const SPECIAL_FOOD_COLOR: Color32 = Color32::from_rgb(255, 0, 255);

/// Render the entire game state
pub fn render_game(painter: &Painter, rect: Rect, game_state: &GameState) {
    let (cell_size, grid_rect) = calculate_grid_layout(rect, game_state.grid);

    // Draw background
    painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 20));

    // Draw grid
    draw_grid(painter, &grid_rect, game_state.grid, cell_size);

    // Draw foods
    #[cfg(not(feature = "multiple_foods"))]
    draw_food(painter, &grid_rect, game_state.food, cell_size);
    
    #[cfg(feature = "multiple_foods")]
    draw_foods(painter, &grid_rect, &game_state.foods, cell_size);

    // Draw snake
    draw_snake(painter, &grid_rect, &game_state.snake, cell_size);

    // Draw HUD
    draw_hud(painter, rect, game_state);
}

/// Calculate cell size and grid rectangle from available space
fn calculate_grid_layout(available_rect: Rect, grid_size: GridSize) -> (f32, Rect) {
    let available_width = available_rect.width() - 20.0; // Padding
    let available_height = available_rect.height() - 100.0; // Padding for HUD

    let cell_width = available_width / grid_size.w as f32;
    let cell_height = available_height / grid_size.h as f32;
    let cell_size = cell_width.min(cell_height);

    let grid_width = cell_size * grid_size.w as f32;
    let grid_height = cell_size * grid_size.h as f32;

    let grid_rect = Rect::from_min_size(
        available_rect.min + egui::vec2(10.0, 10.0),
        egui::vec2(grid_width, grid_height),
    );

    (cell_size, grid_rect)
}

/// Draw the grid outline
fn draw_grid(painter: &Painter, grid_rect: &Rect, grid_size: GridSize, cell_size: f32) {
    // Draw grid lines
    for x in 0..=grid_size.w {
        let x_pos = grid_rect.min.x + x as f32 * cell_size;
        painter.line_segment(
            [
                egui::pos2(x_pos, grid_rect.min.y),
                egui::pos2(x_pos, grid_rect.max.y),
            ],
            Stroke::new(1.0, GRID_COLOR),
        );
    }

    for y in 0..=grid_size.h {
        let y_pos = grid_rect.min.y + y as f32 * cell_size;
        painter.line_segment(
            [
                egui::pos2(grid_rect.min.x, y_pos),
                egui::pos2(grid_rect.max.x, y_pos),
            ],
            Stroke::new(1.0, GRID_COLOR),
        );
    }
}

/// Draw the food
#[cfg(not(feature = "multiple_foods"))]
fn draw_food(painter: &Painter, grid_rect: &Rect, food: Position, cell_size: f32) {
    let cell_rect = cell_rect_for_position(grid_rect, food, cell_size);
    painter.rect_filled(cell_rect.shrink(CELL_MARGIN), 3.0, FOOD_COLOR);
}

/// Draw all foods with different colors based on type
#[cfg(feature = "multiple_foods")]
fn draw_foods(painter: &Painter, grid_rect: &Rect, foods: &[Food], cell_size: f32) {
    for food in foods {
        let cell_rect = cell_rect_for_position(grid_rect, food.position, cell_size);
        let color = match food.food_type {
            FoodType::Normal => NORMAL_FOOD_COLOR,
            FoodType::Golden => GOLDEN_FOOD_COLOR,
            FoodType::Special => SPECIAL_FOOD_COLOR,
        };
        // Special food gets a slightly larger size to make it more noticeable
        let margin = if food.food_type == FoodType::Special {
            CELL_MARGIN * 0.5
        } else {
            CELL_MARGIN
        };
        painter.rect_filled(cell_rect.shrink(margin), 3.0, color);
    }
}

/// Draw the snake
fn draw_snake(painter: &Painter, grid_rect: &Rect, snake: &crate::state::Snake, cell_size: f32) {
    for (i, pos) in snake.body.iter().enumerate() {
        let cell_rect = cell_rect_for_position(grid_rect, *pos, cell_size);
        let color = if i == 0 { HEAD_COLOR } else { SNAKE_COLOR };
        painter.rect_filled(cell_rect.shrink(CELL_MARGIN), 2.0, color);
    }
}

/// Draw the HUD (score, game over message)
fn draw_hud(painter: &Painter, rect: Rect, game_state: &GameState) {
    let hud_y = rect.max.y - 80.0;
    let font = TextStyle::Body.resolve(&Style::default());

    // Score
    let score_text = format!("Score: {}", game_state.score);
    painter.text(
        egui::pos2(rect.min.x + 10.0, hud_y),
        egui::Align2::LEFT_TOP,
        score_text,
        font.clone(),
        Color32::WHITE,
    );

    // Game over message
    if game_state.is_over() {
        let game_over_text = "GAME OVER";
        painter.text(
            egui::pos2(rect.min.x + 10.0, hud_y + 25.0),
            egui::Align2::LEFT_TOP,
            game_over_text,
            font,
            Color32::from_rgb(255, 0, 0),
        );
    }
}

/// Get the rectangle for a grid cell at a given position
fn cell_rect_for_position(grid_rect: &Rect, pos: Position, cell_size: f32) -> Rect {
    let min_x = grid_rect.min.x + pos.x as f32 * cell_size;
    let min_y = grid_rect.min.y + pos.y as f32 * cell_size;
    Rect::from_min_size(egui::pos2(min_x, min_y), egui::vec2(cell_size, cell_size))
}
