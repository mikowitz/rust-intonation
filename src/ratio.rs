//! Provides structs and operators to work with [Ratios][Ratio].
//!

use crate::{
    interval::ApproximateEqualTemperedInterval,
    math::{greatest_prime_factor, reduce},
    play::{play_dyad, play_interval, Play},
};
use num::traits::PrimInt;
use std::time::Duration;
use std::{
    fmt::Display,
    ops::{Div, Mul, Neg},
};

/// Models a ratio of two integral types, defining an interval in just intonation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio<T: PrimInt> {
    pub numer: T,
    pub denom: T,
}

impl<T: PrimInt> Ratio<T> {
    /// Construct a new [Ratio] from two integers.
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
    pub fn new(numer: T, denom: T) -> Self {
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
        let f: f64 = self.into();
        let two: T = num::cast(2i32).unwrap();

        match f {
            f if f < 1. => Self::new(self.numer * two, self.denom).normalize(),
            f if f >= 2. => Self::new(self.numer, self.denom * two).normalize(),
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
        let two: T = num::cast(2).unwrap();
        (Self::new(two, num::one()) / *self).normalize()
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
            e if e == 0 => Self::new(num::one(), num::one()),
            e if e < 0 => self.complement().pow(-exp),
            _ => Self::new(self.numer.pow(exp as u32), self.denom.pow(exp as u32)).normalize(),
        }
    }

    /// Converts the ratio into a tuple pair of an equal tempered interval and
    /// the number of cents difference between the ET interval and the JI ratio
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// # use rust_intonation::interval::EqualTemperedInterval;
    /// let r = Ratio::new(3, 2);
    /// assert_eq!(
    ///   r.to_approximate_equal_tempered_interval(),
    ///   (EqualTemperedInterval::PerfectFifth, 1.955000865387433)
    /// );
    /// ```
    /// This shows that a JI ratio of 3/2 is approximately 2 cents wider than an ET perfect 5th.
    pub fn to_approximate_equal_tempered_interval(&self) -> ApproximateEqualTemperedInterval {
        (*self).into()
    }

    /// Finds the prime limit of the ratio
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(3, 2);
    /// assert_eq!(r.limit(), 3);
    /// ```
    ///
    /// ```rust
    /// # use rust_intonation::ratio::Ratio;
    /// let r = Ratio::new(81, 80);
    /// assert_eq!(r.limit(), 5);
    /// ```
    pub fn limit(&self) -> T {
        greatest_prime_factor(self.numer).max(greatest_prime_factor(self.denom))
    }
}

impl<T: PrimInt> Play for Ratio<T> {
    fn play(&self) {
        // let middle_c = 261.625565;
        let middle_c = 440. * 2.0_f32.powf(-9. / 12.);
        let r: f64 = self.into();
        let ratio_freq = middle_c * r as f32;

        play_interval(middle_c, ratio_freq);

        std::thread::sleep(Duration::from_secs_f32(0.25));

        play_dyad(middle_c, ratio_freq);
    }
}

impl<T: PrimInt + std::fmt::Display> Display for Ratio<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numer, self.denom)
    }
}

impl<T: PrimInt> Neg for Ratio<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.complement()
    }
}

impl<T: PrimInt> Mul<Ratio<T>> for Ratio<T> {
    type Output = Self;

    fn mul(self, rhs: Ratio<T>) -> Self::Output {
        Ratio::<T>::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl<T: PrimInt> Div<Ratio<T>> for Ratio<T> {
    type Output = Self;

    fn div(self, rhs: Ratio<T>) -> Self::Output {
        Ratio::<T>::new(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl<T: PrimInt> From<&Ratio<T>> for f64 {
    fn from(value: &Ratio<T>) -> Self {
        let n: f64 = num::cast(value.numer).unwrap();
        let d: f64 = num::cast(value.denom).unwrap();
        n / d
    }
}

impl<T: PrimInt> From<(T, T)> for Ratio<T> {
    fn from(value: (T, T)) -> Self {
        let (n, d) = value;
        Self::new(n, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::EqualTemperedInterval;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn to_modified_et_interval() {
        let r = Ratio::new(3, 2);
        let i = r.to_approximate_equal_tempered_interval();
        assert_eq!(i.0, EqualTemperedInterval::PerfectFifth);
        assert!((i.1 - 1.955).abs() < 0.0001);
    }

    #[test]
    fn from_tuple() {
        let t = (3, 2);
        assert_eq!(Ratio::from(t), Ratio::new(3, 2));
    }

    #[test]
    fn limit() {
        assert_eq!(Ratio::new(3, 2).limit(), 3);
        assert_eq!(Ratio::new(5, 4).limit(), 5);
        assert_eq!(Ratio::new(8, 5).limit(), 5);
    }

    #[test]
    #[should_panic]
    fn i32_can_overflow() {
        let r = Ratio::new(2147483647, 2147483646);
        let _r2 = r * r;
    }

    #[test]
    fn can_construct_i64_ratio() {
        let r: Ratio<i64> = Ratio::new(2147483647, 2147483646);
        let r2 = r * r;
        assert_eq!(r2, Ratio::new(4611686014132420609, 4611686009837453316));
    }
}
