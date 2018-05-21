extern crate num_traits;

use num_traits::{One, Zero};
use std::ops::{Add, Mul};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x: x, y: y }
    }
}

impl<T: Zero> Vector2<T> {
    pub fn origin() -> Self {
        Vector2 {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Vector2<T>
where
    T: Zero + One,
{
    pub fn unit_x() -> Self {
        Vector2 {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Vector2 {
            x: T::zero(),
            y: T::one(),
        }
    }
}

impl<T, U> Add<Vector2<U>> for Vector2<T>
where
    T: Add<U>,
{
    type Output = Vector2<T::Output>;
    fn add(self, rhs: Vector2<U>) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, U> Mul<U> for Vector2<T>
where
    T: Mul<U>,
    U: Clone,
{
    type Output = Vector2<T::Output>;
    fn mul(self, rhs: U) -> Self::Output {
        Vector2::new(self.x * rhs.clone(), self.y * rhs)
    }
}
