use std::ops::{Add, Mul};

#[cfg(test)]
mod tests;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn sum_prod<T, U>(a: T, b: U) -> (<T as Add<U>>::Output, <T as Mul<U>>::Output)
where
    T: Add<U> + Mul<U> + Clone,
    U: Clone,
{
    (a.clone() + b.clone(), a * b)
}

trait ToOptionExt {
    fn into_option<T>(&self, val: T) -> Option<T>;
}

impl ToOptionExt for bool {
    fn into_option<T>(&self, val: T) -> Option<T> {
        match self {
            true => Some(val),
            false => None,
        }
    }
}
