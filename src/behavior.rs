use amethyst_core::specs::{Component, DenseVecStorage, Entity};

pub enum Behavior {
    Flee(Entity),
    Evade(Entity, f32),
}

impl Component for Behavior {
    type Storage = DenseVecStorage<Self>;
}
