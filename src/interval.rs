//! Operations for converting between JI ratios and approximations of ET (cent-based) intervals

use crate::ratio::Ratio;
use num::traits::PrimInt;

macro_rules! ji_interval {
    ($name:ident $n:tt/$d:tt) => {
        #[doc = concat!("A just intonation interval with a ratio of `", stringify!($n), "/", stringify!($d), "`.")]
        pub const $name: (i32, i32) = ($n, $d);
    };
}

ji_interval! { UNISON 1/1 }
ji_interval! { MAJOR_SECOND 9/8 }
ji_interval! { MAJOR_THIRD 5/4 }
ji_interval! { PERFECT_FOURTH 4/3 }
ji_interval! { PERFECT_FIFTH 3/2 }
ji_interval! { MAJOR_SIXTH 5/3 }
ji_interval! { MAJOR_SEVEN 15/8 }
ji_interval! { OCTAVE 2/1 }
ji_interval! { SYNTONIC_COMMA 81/80 }

#[derive(Debug, PartialEq)]
pub enum EqualTemperedInterval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
}

impl From<f64> for EqualTemperedInterval {
    fn from(value: f64) -> Self {
        match (value / 100.) % 12. {
            n if n == 0. => Self::PerfectUnison,
            n if n == 1. => Self::MinorSecond,
            n if n == 2. => Self::MajorSecond,
            n if n == 3. => Self::MinorThird,
            n if n == 4. => Self::MajorThird,
            n if n == 5. => Self::PerfectFourth,
            n if n == 6. => Self::AugmentedFourth,
            n if n == 7. => Self::PerfectFifth,
            n if n == 8. => Self::MinorSixth,
            n if n == 9. => Self::MajorSixth,
            n if n == 10. => Self::MinorSeventh,
            n if n == 11. => Self::MajorSeventh,
            _ => panic!(),
        }
    }
}

/// Describes the approximation of an equal tempered interval as a tuple
/// pair of the named ET interval and a difference from ET, given in cents.
pub type ApproximateEqualTemperedInterval = (EqualTemperedInterval, f64);

impl<T: PrimInt> From<Ratio<T>> for ApproximateEqualTemperedInterval {
    fn from(value: Ratio<T>) -> Self {
        let f: f64 = (&value.normalize()).into();
        let ji_cents: f64 = 1200. * f.log2();

        let et_cents = (ji_cents / 100.).round() * 100.;

        (et_cents.into(), ji_cents - et_cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::Ratio;
    use EqualTemperedInterval::*;

    #[test]
    fn unison_and_octaves() {
        let r1 = Ratio::new(1, 1);
        let i1: ApproximateEqualTemperedInterval = r1.into();
        assert_eq!(i1, (PerfectUnison, 0.));

        let r2 = Ratio::new(2, 1);
        let i2: ApproximateEqualTemperedInterval = r2.into();
        assert_eq!(i2, (PerfectUnison, 0.));

        let r3 = Ratio::new(1, 2);
        let i3: ApproximateEqualTemperedInterval = r3.into();
        assert_eq!(i3, (PerfectUnison, 0.));
    }

    #[test]
    fn perfect_fifth() {
        let r = Ratio::new(3, 2);
        let i: ApproximateEqualTemperedInterval = r.into();
        assert_eq!(i.0, PerfectFifth);
        assert!((i.1 - 1.955).abs() < 0.001);
    }
}
