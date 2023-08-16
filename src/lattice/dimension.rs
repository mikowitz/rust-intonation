use super::dimension_bounds::LatticeDimensionBounds;
use crate::ratio::Ratio;

/// Models one dimension of a lattice, defining the [Ratio] by which to extend the dimension,
/// as well as the rules for [bounding][LatticeDimensionBounds] the dimension.
pub struct LatticeDimension {
    pub ratio: Ratio,
    pub bounds: LatticeDimensionBounds,
}

impl LatticeDimension {
    pub fn new(ratio: Ratio, bounds: LatticeDimensionBounds) -> Self {
        Self { ratio, bounds }
    }

    /// Indexes into the dimension, based on the [bounding rules][LatticeDimensionBounds] defined
    /// for the dimension.
    pub fn at(&self, index: i32) -> Ratio {
        let index = self.bounds.resolve_index(index);
        self.ratio.pow(index)
    }
}

#[cfg(test)]
mod tests {
    use super::LatticeDimensionBounds::*;
    use super::*;

    #[test]
    fn at_for_unbounded_dimension() {
        let dim = LatticeDimension::new(Ratio::new(3, 2), Infinite);

        assert_eq!(dim.at(0), Ratio::new(1, 1));
        assert_eq!(dim.at(1), Ratio::new(3, 2));
        assert_eq!(dim.at(2), Ratio::new(9, 8));
        assert_eq!(dim.at(-1), Ratio::new(4, 3));
    }

    #[test]
    fn at_for_length_bounded_dimension() {
        let dim = LatticeDimension::new(Ratio::new(3, 2), LengthBounded(2));

        assert_eq!(dim.at(0), Ratio::new(1, 1));
        assert_eq!(dim.at(1), Ratio::new(3, 2));
        assert_eq!(dim.at(2), Ratio::new(1, 1));
        assert_eq!(dim.at(-1), Ratio::new(3, 2));
    }

    #[test]
    fn at_for_negative_length_bounded_dimension() {
        let dim = LatticeDimension::new(Ratio::new(3, 2), LengthBounded(-2));

        assert_eq!(dim.at(0), Ratio::new(1, 1));
        assert_eq!(dim.at(1), Ratio::new(4, 3));
        assert_eq!(dim.at(2), Ratio::new(1, 1));
        assert_eq!(dim.at(-1), Ratio::new(4, 3));
        assert_eq!(dim.at(-2), Ratio::new(1, 1));
    }

    #[test]
    fn at_for_range_bounded_dimension() {
        let dim = LatticeDimension::new(Ratio::new(3, 2), RangeBounded(-2, 3));

        assert_eq!(dim.at(0), Ratio::new(1, 1));
        assert_eq!(dim.at(1), Ratio::new(3, 2));
        assert_eq!(dim.at(2), Ratio::new(9, 8));
        assert_eq!(dim.at(3), Ratio::new(27, 16));
        assert_eq!(dim.at(4), Ratio::new(16, 9));
        assert_eq!(dim.at(-1), Ratio::new(4, 3));
        assert_eq!(dim.at(-2), Ratio::new(16, 9));
        assert_eq!(dim.at(-3), Ratio::new(27, 16));
    }
}
