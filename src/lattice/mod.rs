//! Tools for constructing and indexing an n-dimensional lattice of just intonation intervals

pub mod dimension;
pub mod dimension_bounds;

use crate::ratio::Ratio;
pub use dimension::LatticeDimension;
pub use dimension_bounds::LatticeDimensionBounds;

/// Models an n-dimensional just intonation ratio lattice, constructed from a vector
/// of [LatticeDimensions][LatticeDimension].
pub struct Lattice {
    pub dimensions: Vec<LatticeDimension>,
}

impl Lattice {
    pub fn new(dimensions: Vec<LatticeDimension>) -> Self {
        Self { dimensions }
    }

    pub fn at(&self, indices: &[i32]) -> Ratio {
        self.dimensions
            .iter()
            .zip(indices.iter())
            .map(|(dim, &index)| dim.at(index))
            .fold(Ratio::new(1, 1), |r, acc| r * acc)
    }
}

#[cfg(test)]
mod tests {
    use super::dimension_bounds::LatticeDimensionBounds::*;
    use super::*;
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
}
