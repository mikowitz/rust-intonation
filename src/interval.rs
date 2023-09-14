//! Operations for converting between JI ratios and approximations of ET (cent-based) intervals

use crate::play::{play_dyad, Play};
use crate::ratio::Ratio;
use crate::temperaments::edo::EdoInterval;
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
pub enum TwelveEDOInterval {
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

impl Play for TwelveEDOInterval {
    fn play(&self) {
        let middle_c = 440. * 2.0_f32.powf(-9. / 12.);
        let et_steps: usize = self.into();
        let et_steps = et_steps as f32;
        let et_freq = middle_c * 2f32.powf(et_steps / 12.);

        play_dyad(middle_c, et_freq);
    }
}

impl From<&TwelveEDOInterval> for usize {
    fn from(value: &TwelveEDOInterval) -> Self {
        match value {
            TwelveEDOInterval::PerfectUnison => 0,
            TwelveEDOInterval::MinorSecond => 1,
            TwelveEDOInterval::MajorSecond => 2,
            TwelveEDOInterval::MinorThird => 3,
            TwelveEDOInterval::MajorThird => 4,
            TwelveEDOInterval::PerfectFourth => 5,
            TwelveEDOInterval::AugmentedFourth => 6,
            TwelveEDOInterval::PerfectFifth => 7,
            TwelveEDOInterval::MinorSixth => 8,
            TwelveEDOInterval::MajorSixth => 9,
            TwelveEDOInterval::MinorSeventh => 10,
            TwelveEDOInterval::MajorSeventh => 11,
        }
    }
}

impl From<f64> for TwelveEDOInterval {
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
pub type Approximate12EDOInterval = (TwelveEDOInterval, f64);

impl<T: PrimInt> From<Ratio<T>> for Approximate12EDOInterval {
    fn from(value: Ratio<T>) -> Self {
        let f: f64 = (&value).into();
        let ji_cents: f64 = 1200. * f.log2();

        let et_cents = (ji_cents / 100.).round() * 100.;

        (et_cents.into(), ji_cents - et_cents)
    }
}

impl<'a> From<EdoInterval<'a>> for Approximate12EDOInterval {
    fn from(value: EdoInterval<'a>) -> Self {
        let non_12_cents: f64 = value.cents as f64;
        let et_12_cents: f64 = (non_12_cents / 100.).round() * 100.;
        (et_12_cents.into(), non_12_cents - et_12_cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::Ratio;
    use pretty_assertions::assert_eq;
    use TwelveEDOInterval::*;

    #[test]
    fn unison() {
        let r1 = Ratio::new(1, 1);
        let i1: Approximate12EDOInterval = r1.into();
        assert_eq!(i1, (PerfectUnison, 0.));
    }

    #[test]
    fn perfect_fifth() {
        let r = Ratio::new(3, 2);
        let i: Approximate12EDOInterval = r.into();
        assert_eq!(i.0, PerfectFifth);
        assert!((i.1 - 1.955).abs() < 0.001);
    }
}
