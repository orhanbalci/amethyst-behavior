use amethyst_core::specs::{Component, DenseVecStorage, Entity};

pub enum Behavior {
    Flee(Entity),
    Evade(Entity, f32),            //target, time_to_target
    Arrive(Entity, f32, f32, f32), //target, tolerance, deceleration radius, time_to_target
    Pursue(Entity, f32),           //target, max_prediction_time
    Seek(Entity),                  //target
}

impl Component for Behavior {
    type Storage = DenseVecStorage<Self>;
}
