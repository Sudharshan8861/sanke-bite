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
