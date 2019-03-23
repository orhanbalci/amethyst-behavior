use amethyst_core::specs::{Entities, Join, ReadStorage, System};
use amethyst_core::Transform;
use nalgebra::{distance, Point3, Vector3};

use super::{Behavior, Limiter, SteeringAcceleration, Velocity};

pub struct ArriveSystem;

impl<'s> System<'s> for ArriveSystem {
    type SystemData = (
        ReadStorage<'s, Behavior>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadStorage<'s, Limiter>,
        ReadStorage<'s, Velocity>,
    );

    fn run(&mut self, (behaviors, transforms, entities, limiters, velocities): Self::SystemData) {
        for (b, owner) in (&behaviors, &entities).join() {
            match b {
                Behavior::Arrive(target, tolerance, deceleration_radius, time_to_target) => {
                    let mut sa: SteeringAcceleration<f32> = SteeringAcceleration::default();
                    sa.linear = transforms.get(*target).cloned().unwrap().translation()
                        - transforms.get(owner).cloned().unwrap().translation();

                    let to_target = distance(&Point3::from(sa.linear), &Point3::origin());

                    if to_target <= *tolerance {
                        sa.set_zero();
                    }

                    let mut target_speed = 1.0;
                    if let Some(limiter) = limiters.get(owner) {
                        match limiter {
                            Limiter::LinearAccelerationLimiter(lin_limit) => {
                                target_speed = *lin_limit
                            }
                        }
                    }
                    if to_target <= *deceleration_radius {
                        target_speed *= to_target / deceleration_radius;
                    }

                    sa.linear
                        .component_mul_assign(&Vector3::from_element(target_speed / to_target));

                    if let Some(vel) = velocities.get(owner) {
                        sa.linear -= vel.velocity
                    }

                    sa.linear
                        .component_mul_assign(&Vector3::from_element(1.0 / time_to_target));
                    sa.angular = 0.0;
                }
                _ => (),
            }
        }
    }
}
