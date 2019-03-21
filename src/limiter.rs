use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub enum Limiter<T:Real>{
    LinearAccelerationLimiter(max_linear_acceleration : T)
}