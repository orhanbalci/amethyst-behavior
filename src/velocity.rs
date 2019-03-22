use amethyst_core::specs::{Component, DenseVecStorage};
use nalgebra::Vector3;

pub struct Velocity {
    pub velocity: Vector3<f32>,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
