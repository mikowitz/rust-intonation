//! Tools for constructing and indexing an n-dimensional lattice of just intonation intervals

pub mod dimension;
pub mod dimension_bounds;

use crate::ratio::Ratio;
pub use dimension::LatticeDimension;
pub use dimension_bounds::LatticeDimensionBounds;

use num::traits::PrimInt;

/// Models an n-dimensional just intonation ratio lattice, constructed from a vector
/// of [LatticeDimensions][LatticeDimension].
pub struct Lattice<T: PrimInt> {
    pub dimensions: Vec<LatticeDimension<T>>,
}

impl<T: PrimInt> Lattice<T> {
    /// Construct a new [Lattice] from a vector of [LatticeDimenions][LatticeDimension]
    pub fn new(dimensions: Vec<LatticeDimension<T>>) -> Self {
        Self { dimensions }
    }

    pub fn at(&self, indices: &[i32]) -> Ratio<T> {
        self.dimensions
            .iter()
            .zip(indices.iter())
            .map(|(dim, &index)| dim.at(index))
            .fold(Ratio::new(num::one(), num::one()), |r, acc| acc * r)
    }
}

#[cfg(test)]
mod tests {
    use super::dimension_bounds::LatticeDimensionBounds::*;
    use super::*;
    use crate::lattice::dimension::LatticeDimension;
    use crate::ratio::Ratio;

    #[test]
    fn one_dimensional_unbounded_lattice() {
        let l = Lattice::new(vec![LatticeDimension::new(Ratio::new(3, 2), Infinite)]);

        assert_eq!(l.at(&[0]), Ratio::new(1, 1));
        assert_eq!(l.at(&[1]), Ratio::new(3, 2));
    }

    #[test]
    fn two_dimensional_unbounded_lattice() {
        let l = Lattice::new(vec![
            LatticeDimension::new(Ratio::new(3, 2), Infinite),
            LatticeDimension::new(Ratio::new(5, 4), Infinite),
        ]);

        assert_eq!(l.at(&[0]), Ratio::new(1, 1));
        assert_eq!(l.at(&[1]), Ratio::new(3, 2));
        assert_eq!(l.at(&[1, 0]), Ratio::new(3, 2));
        assert_eq!(l.at(&[1, 1]), Ratio::new(15, 8));
    }

    #[test]
    #[should_panic]
    fn default_i32_lattice_panics() {
        let l = Lattice::new(vec![
            LatticeDimension::new(Ratio::new(3, 2), Infinite),
            LatticeDimension::new(Ratio::new(5, 4), Infinite),
            LatticeDimension::new(Ratio::new(7, 4), Infinite),
        ]);

        l.at(&[7, 7, 7]);
    }

    #[test]
    fn can_create_an_i64_lattice() {
        let l: Lattice<i64> = Lattice::new(vec![
            LatticeDimension::new(Ratio::new(3, 2), Infinite),
            LatticeDimension::new(Ratio::new(5, 4), Infinite),
            LatticeDimension::new(Ratio::new(7, 4), Infinite),
        ]);

        let r = l.at(&[7, 7, 7]);
        assert_eq!(r, Ratio::new(140710042265625, 70368744177664));
    }
}
