use amethyst_core::specs::{Entities, Join, ReadStorage, System};
use amethyst_core::Transform;
use nalgebra::Vector3;

use super::{Behavior, Limiter, SteeringAcceleration};

pub struct SeekSystem;

impl<'s> System<'s> for SeekSystem {
    type SystemData = (
        ReadStorage<'s, Behavior>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, Limiter>,
    );

    fn run(&mut self, (behaviors, transforms, entities, limiters): Self::SystemData) {
        for (b, e) in (&behaviors, &entities).join() {
            match b {
                Behavior::Seek(target) => {
                    let mut sa: SteeringAcceleration<f32> = SteeringAcceleration::default();
                    let mut position_diff = transforms.get(*target).cloned().unwrap().translation()
                        - transforms.get(e).cloned().unwrap().translation();
                    position_diff = position_diff.normalize();
                    if let Some(limiter) = limiters.get(e) {
                        match limiter {
                            Limiter::LinearAccelerationLimiter(lin_limit) => position_diff
                                .component_mul_assign(&Vector3::from_element(*lin_limit)),
                        }
                    }
                    sa.linear = position_diff;
                }
                _ => (),
            }
        }
    }
}
