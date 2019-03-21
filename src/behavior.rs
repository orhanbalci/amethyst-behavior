use amethyst_core::specs::{Component, DenseVecStorage, Entity};

pub enum Behavior {
    Flee(Entity),
}

impl Component for Behavior {
    type Storage = DenseVecStorage<Self>;
}
