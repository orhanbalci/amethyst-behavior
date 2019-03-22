use amethyst_core::specs::{Entities, Join, ReadStorage, System, WriteStorage};
use amethyst_core::Transform;
use nalgebra::Vector3;

use super::{Behavior, Limiter, SteeringAcceleration};

pub struct FleeSystem;

impl<'s> System<'s> for FleeSystem {
    type SystemData = (
        ReadStorage<'s, Behavior>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, Limiter>,
    );

    fn run(&mut self, (behaviors, transforms, entities, limiters): Self::SystemData) {
        for (b, e) in (&behaviors, &entities).join() {
            match b {
                Behavior::Flee(target) => {
                    let mut sa: SteeringAcceleration<f32> = SteeringAcceleration::default();
                    sa.linear = (transforms.get(e).cloned().unwrap().translation()
                        - transforms.get(*target).cloned().unwrap().translation())
                    .normalize();
                    if let Some(limiter) = limiters.get(e) {
                        match limiter {
                            Limiter::LinearAccelerationLimiter(lin_limit) => sa
                                .linear
                                .component_mul_assign(&Vector3::from_element(*lin_limit)),
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
