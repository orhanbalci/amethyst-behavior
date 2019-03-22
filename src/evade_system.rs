use amethyst_core::specs::{Entities, Join, ReadStorage, System, WriteStorage};
use amethyst_core::Transform;
use nalgebra::{distance_squared, Point3, Vector3};

use super::{Behavior, Limiter, SteeringAcceleration, Velocity};

pub struct EvadeSystem;

impl<'s> System<'s> for EvadeSystem {
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
                Behavior::Evade(target, max_prediction_time) => {
                    let square_distance = distance_squared(
                        &Point3::from(
                            transforms.get(*target).cloned().unwrap().translation()
                                - transforms.get(owner).cloned().unwrap().translation(),
                        ),
                        &Point3::origin(),
                    );

                    let square_speed = match velocities.get(owner) {
                        Some(vel) => {
                            distance_squared(&Point3::from(vel.velocity), &Point3::origin())
                        }
                        None => 0.0,
                    };

                    let mut prediction_time = *max_prediction_time;

                    if square_speed > 0.0 {
                        let square_prediction_time = square_distance / square_speed;
                        if square_prediction_time < max_prediction_time * max_prediction_time {
                            prediction_time = square_prediction_time.sqrt();
                        }
                    }

                    let mut sa: SteeringAcceleration<f32> = SteeringAcceleration::default();
                    sa.linear = *transforms.get(*target).cloned().unwrap().translation();
                    sa.mul_add(
                        SteeringAcceleration::new(velocities.get(*target).unwrap().velocity, 0.0),
                        prediction_time,
                    );
                    sa.linear -= transforms.get(owner).cloned().unwrap().translation();
                    sa.linear = sa.linear.normalize();

                    if let Some(limiter) = limiters.get(owner) {
                        match limiter {
                            Limiter::LinearAccelerationLimiter(lin_limit) => sa
                                .linear
                                .component_mul_assign(&-Vector3::from_element(*lin_limit)),
                        }
                    }
                    sa.angular = 0.0;
                }
                _ => (),
            }
        }
    }
}
