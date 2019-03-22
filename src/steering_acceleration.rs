use alga::general::Real;
use nalgebra::{distance_squared, Point3, Vector3};
use num_traits::identities::Zero;

use std::cell::RefCell;
use std::rc::Rc;

/// Represents result of a steering behaviour computation. User can aggregate
/// more than one behaviour result into single acceleration struct.
#[derive(Debug, PartialEq)]
pub struct SteeringAcceleration<T: Real> {
    /// linear acceleration component
    pub linear: Vector3<T>,
    /// angular acceleration component
    pub angular: T,
}

impl<T: Real> SteeringAcceleration<T> {
    pub fn default() -> SteeringAcceleration<T> {
        SteeringAcceleration {
            linear: Vector3::zero(),
            angular: T::zero(),
        }
    }
    /// Creates a steering acceleration struct using given linear and angular components
    pub fn new(
        linear_acceleration: Vector3<T>,
        angular_acceleration: T,
    ) -> SteeringAcceleration<T> {
        SteeringAcceleration {
            linear: linear_acceleration,
            angular: angular_acceleration,
        }
    }

    /// Tests whether both linear and angular acceleration compenents are zero
    pub fn is_zero(&self) -> bool {
        self.angular.is_zero() && self.linear.is_zero()
    }

    /// Sets both compononents to zero
    pub fn set_zero(&mut self) -> &mut Self {
        self.angular = T::zero();
        self.linear = Vector3::zero();
        self
    }

    ///
    pub fn add(&mut self, other: SteeringAcceleration<T>) -> &mut Self {
        self.angular += other.angular;
        self.linear += other.linear;
        self
    }

    ///
    pub fn scl(&mut self, scale: T) -> &mut Self {
        self.angular *= scale;
        self.linear
            .component_mul_assign(&Vector3::from_element(scale));
        self
    }

    ///
    pub fn mul_add(&mut self, other: SteeringAcceleration<T>, scale: T) -> &mut Self {
        self.angular += other.angular * scale;
        self.linear += other.linear.component_mul(&Vector3::from_element(scale));
        self
    }

    ///
    pub fn calculate_square_magnitude(&self) -> T {
        distance_squared(&Point3::from(self.linear), &Point3::origin())
            + self.angular * self.angular
    }

    ///
    pub fn calculate_magnitude(&self) -> T {
        self.calculate_square_magnitude().sqrt()
    }
}

pub trait SteeringAccelerationCalculator<T: Real> {
    fn calculate_steering(
        &mut self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        self.calculate_real_steering(steering_acceleration.clone());
        steering_acceleration
    }

    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>>;
    // fn set_enabled(&mut self, is_enabled : bool);
}

#[cfg(test)]
mod test {
    use super::SteeringAcceleration;
    use nalgebra::Vector3;
    #[test]
    fn is_zero_positive() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 2.0, 3.0), 5.0f32);
        acceleration.set_zero();
        assert!(acceleration.is_zero());
    }

    #[test]
    fn is_zero_negative() {
        let acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 2.0, 3.0), 5.0f32);
        assert_eq!(acceleration.is_zero(), false);
    }

    #[test]
    fn add() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0f32);
        let acceleration2 = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0f32);
        acceleration.add(acceleration2);
        assert_eq!(
            SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0f32),
            acceleration
        );
    }

    #[test]
    fn scl() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0f32);
        acceleration.scl(2.0f32);
        assert_eq!(
            SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0),
            acceleration
        );
    }

    #[test]
    fn calculate_square_magnitude() {
        let acceleration = SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0f32);
        assert_eq!(16f32, acceleration.calculate_square_magnitude());
    }

    #[test]
    fn calculate_magnitude() {
        let acceleration = SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0f32);
        assert_eq!(4f32, acceleration.calculate_magnitude());
    }

    #[test]
    fn mul_add() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0);
        let acceleration2 = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0);
        acceleration.mul_add(acceleration2, 2.0);
        assert_eq!(
            SteeringAcceleration::new(Vector3::new(3.0f32, 3.0, 3.0), 3.0),
            acceleration
        );
    }
}
