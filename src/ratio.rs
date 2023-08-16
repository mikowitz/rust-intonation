//! Provides structs and operators to work with [Ratios][Ratio].
//!

use crate::math::reduce;
use std::ops::{Div, Mul, Neg};

/// Models a ratio of two integers, defining an interval in just intonation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio {
    pub numer: i32,
    pub denom: i32,
}

impl Neg for Ratio {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.complement()
    }
}

impl Mul<Ratio> for Ratio {
    type Output = Self;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Self::Output::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl Div<Ratio> for Ratio {
    type Output = Self;

    fn div(self, rhs: Ratio) -> Self::Output {
        Self::Output::new(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl From<&Ratio> for f32 {
    fn from(value: &Ratio) -> Self {
        value.numer as f32 / value.denom as f32
    }
}

impl Ratio {
    /// Construct a new [Ratio] from two [i32].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(3, 2);
    /// ```
    /// [`Ratio::new()`] will reduce the given fraction to its smallest representation
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(5, 10);
    /// assert_eq!(r.numer, 1);
    /// assert_eq!(r.denom, 2);
    /// ```
    pub fn new(numer: i32, denom: i32) -> Self {
        let (numer, denom) = reduce(numer, denom);
        Self { numer, denom }
    }

    /// Normalizes the [Ratio] to an absolute value in the range [1, 2)
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(1, 2);
    /// assert_eq!(r.normalize(), Ratio::new(1, 1));
    /// ```
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(2, 1);
    /// assert_eq!(r.normalize(), Ratio::new(1, 1));
    /// ```
    pub fn normalize(&self) -> Self {
        let f: f32 = self.into();

        match f {
            f if f < 1. => Self::new(self.numer * 2, self.denom).normalize(),
            f if f >= 2. => Self::new(self.numer, self.denom * 2).normalize(),
            _ => Self::new(self.numer, self.denom),
        }
    }

    /// Returns the [Ratio] that, when multiplied by the given argument, gives `2/1`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(3, 2);
    /// assert_eq!(r.complement(), Ratio::new(4, 3));
    /// ```
    pub fn complement(&self) -> Self {
        (Self::new(2, 1) / *self).normalize()
    }

    /// Raises the given [Ratio] to the given integral power
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(3, 2);
    /// assert_eq!(r.pow(0), Ratio::new(1, 1));
    /// assert_eq!(r.pow(1), Ratio::new(3, 2));
    /// assert_eq!(r.pow(2), Ratio::new(9, 8));
    /// assert_eq!(r.pow(-2), Ratio::new(16, 9));
    /// ```
    pub fn pow(&self, exp: i32) -> Self {
        match exp {
            e if e == 0 => Self::new(1, 1),
            e if e < 0 => self.complement().pow(-exp),
            _ => Self::new(self.numer.pow(exp as u32), self.denom.pow(exp as u32)).normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_simple_ratio() {
        let r = Ratio::new(3, 2);

        assert_eq!(r.numer, 3);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn reduces_ratio() {
        let r = Ratio::new(6, 4);

        assert_eq!(r.numer, 3);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn normalize() {
        let r1 = Ratio::new(1, 2);
        let r2 = Ratio::new(9, 4);

        assert_eq!(r1.normalize(), Ratio::new(1, 1));
        assert_eq!(r2.normalize(), Ratio::new(9, 8));
    }

    #[test]
    fn mul() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(r1 * r2, Ratio::new(27, 16));
        assert_eq!(r2 * r1, Ratio::new(27, 16));
    }

    #[test]
    fn div() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(r1 / r2, Ratio::new(4, 3));
        assert_eq!(r2 / r1, Ratio::new(3, 4));
    }

    #[test]
    fn complement() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(r1.complement(), Ratio::new(4, 3));
        assert_eq!(r2.complement(), Ratio::new(16, 9));
    }

    #[test]
    fn complement_operator() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(-r1, Ratio::new(4, 3));
        assert_eq!(-r2, Ratio::new(16, 9));
    }

    #[test]
    fn pow() {
        let r1 = Ratio::new(3, 2);

        assert_eq!(r1.pow(0), Ratio::new(1, 1));
        assert_eq!(r1.pow(1), Ratio::new(3, 2));
        assert_eq!(r1.pow(2), Ratio::new(9, 8));
        assert_eq!(r1.pow(-2), Ratio::new(16, 9));
    }
}
