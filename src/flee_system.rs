
use amethyst::ecs::{System, SystemData, Entities, WriteStorage, Join, ReadStorage};
use amethyst::core::Transform;
use crate::SteeringAcceleration;

use super::Behavior;

pub struct FleeSystem;

impl <'s> System<'s> for FleeSystem {
    type SystemData = (
        ReadStorage<'s, Behavior>,
        WriteStorage<'s, Transform>,
        Entities<'s>
    );

    fn run(&mut self, (behaviors, mut transforms, entities) : Self::SystemData) {
        for (b, e) in (&behaviors, &entities).join() {
            match b {
                Behavior::Flee(target) => {
                    let mut sa : SteeringAcceleration<f32> = SteeringAcceleration::default();
                    sa.linear = (transforms.get(e).cloned().unwrap().translation() -
                                                         transforms.get(*target).cloned().unwrap().translation())
            .normalize()
            .append_scaling(0.1);               
                },
                _ => ()
            }

        }
    }
}