#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridSize {
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // Will be used in systems module
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // Will be used in systems module
pub struct Tick(pub u64);

#[cfg(feature = "multiple_foods")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoodType {
    Normal,
    Golden,
    Special,
}

#[cfg(feature = "multiple_foods")]
impl FoodType {
    pub fn point_value(&self) -> u32 {
        match self {
            FoodType::Normal => 1,
            FoodType::Golden => 5,
            FoodType::Special => 10,
        }
    }
}

#[cfg(feature = "multiple_foods")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Food {
    pub position: Position,
    pub food_type: FoodType,
}

#[cfg(feature = "powerups")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerUpType {
    Slow,    // Reduces movement speed (skip steps)
    Fast,    // Increases movement speed (double steps)
    Poison,  // Reduces snake length
}

#[cfg(feature = "powerups")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerUp {
    pub position: Position,
    pub power_type: PowerUpType,
    pub remaining_duration: u32, // Number of ticks remaining
}

#[cfg(feature = "powerups")]
impl PowerUp {
    /// Initial duration for each power-up type
    pub fn initial_duration(&self) -> u32 {
        match self.power_type {
            PowerUpType::Slow => 20,   // Lasts 20 ticks
            PowerUpType::Fast => 15,   // Lasts 15 ticks
            PowerUpType::Poison => 0,  // Poison applies immediately, no duration
        }
    }
}
