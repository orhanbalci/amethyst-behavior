use amethyst_core::specs::{Component, DenseVecStorage};

pub enum Limiter {
    LinearAccelerationLimiter(f32),
}

impl Component for Limiter {
    type Storage = DenseVecStorage<Self>;
}
