use crate::interval::Approximate12EDOInterval;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edo {
    pub divisions: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdoInterval<'a> {
    edo: &'a Edo,
    steps: u32,
    pub cents: f32,
}

impl Edo {
    pub fn new(divisions: u32) -> Self {
        Self { divisions }
    }

    pub fn interval(&self, steps: u32) -> EdoInterval {
        EdoInterval::new(self, steps)
    }
}

impl<'a> EdoInterval<'a> {
    pub fn new(edo: &'a Edo, steps: u32) -> EdoInterval<'a> {
        let cents = 1200. * (steps as f32) / (edo.divisions as f32);
        Self { edo, steps, cents }
    }

    pub fn to_approximate_12_edo_interval(&self) -> Approximate12EDOInterval {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::TwelveEDOInterval;

    use super::*;

    #[test]
    fn interval() {
        let twelve = Edo::new(12);

        let fifth = twelve.interval(7);

        assert_eq!(fifth.steps, 7);
        assert_eq!(fifth.edo, &twelve);
        assert_eq!(fifth.cents, 700.);

        let sixth = twelve.interval(9);
        assert_eq!(sixth.steps, 9);
        assert_eq!(sixth.edo, &twelve);
        assert_eq!(sixth.cents, 900.);
    }

    #[test]
    fn non_12_edo() {
        let fifty_three = Edo::new(53);

        let fifth = fifty_three.interval(31);

        assert_eq!(fifth.steps, 31);
        assert_eq!(fifth.edo, &fifty_three);
        assert_eq!(fifth.cents, 701.8868);
    }

    #[test]
    fn closest_12_edo() {
        let twelve = Edo::new(12);

        let fifth = twelve.interval(7);

        let approx = fifth.to_approximate_12_edo_interval();

        assert_eq!(approx.0, TwelveEDOInterval::PerfectFifth);
        assert_eq!(approx.1, 0.);
    }
}
