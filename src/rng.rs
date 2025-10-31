/// Trait for random number generators
pub trait RngLike {
    fn next_u32(&mut self) -> u32;
}

/// A deterministic random number generator using a seeded algorithm
#[derive(Clone)]
pub struct Seeded(u64);

impl Seeded {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
}

impl RngLike for Seeded {
    fn next_u32(&mut self) -> u32 {
        // Simple xorshift64* algorithm (fast, deterministic for tests)
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545F4914F6CDD1D) >> 32) as u32
    }
}
