use crate::{rng::RngLike, state::GameState, types::GridSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub grid: GridSize,
    pub speed: u32, // logical speed units (e.g., ticks per second)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingsError {
    InvalidGridWidth(i32),
    InvalidGridHeight(i32),
    InvalidSpeed(u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self { grid: GridSize { w: 10, h: 10 }, speed: 10 }
    }
}

impl Settings {
    pub fn new(grid: GridSize, speed: u32) -> Result<Self, SettingsError> {
        let candidate = Self { grid, speed };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.grid.w <= 0 { return Err(SettingsError::InvalidGridWidth(self.grid.w)); }
        if self.grid.h <= 0 { return Err(SettingsError::InvalidGridHeight(self.grid.h)); }
        // Allow a reasonable speed range for tests and UI; can be adjusted later
        if self.speed == 0 || self.speed > 60 { return Err(SettingsError::InvalidSpeed(self.speed)); }
        Ok(())
    }

    pub fn with_grid(mut self, grid: GridSize) -> Result<Self, SettingsError> {
        self.grid = grid;
        self.validate()?;
        Ok(self)
    }

    pub fn with_speed(mut self, speed: u32) -> Result<Self, SettingsError> {
        self.speed = speed;
        self.validate()?;
        Ok(self)
    }

    pub fn apply_to_new_game<R: RngLike>(&self, rng: R) -> GameState {
        // Validation is expected to be enforced by constructors; in case of misuse, clamp at runtime isn't applied here.
        GameState::new(self.grid, rng)
    }
}

#[derive(Clone, Debug)]
pub struct SettingsStore {
    settings: Settings,
}

impl SettingsStore {
    pub fn new(settings: Settings) -> Result<Self, SettingsError> {
        settings.validate()?;
        Ok(Self { settings })
    }

    pub fn default() -> Self { Self { settings: Settings::default() } }

    pub fn get(&self) -> Settings { self.settings }

    pub fn update(&mut self, new_settings: Settings) -> Result<(), SettingsError> {
        new_settings.validate()?;
        self.settings = new_settings;
        Ok(())
    }
}


