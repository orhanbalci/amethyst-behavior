use alga::general::Real;
use amethyst_core::specs::{Component, DenseVecStorage};

pub enum Limiter<T: Real> {
    LinearAccelerationLimiter(T),
}

impl<T: Real> Component for Limiter<T> {
    type Storage = DenseVecStorage<Self>;
}
