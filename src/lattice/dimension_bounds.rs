use crate::math::sign_preserving_mod;

/// Models the different possibilities for a lattice dimension's bounding
#[derive(Clone, Copy, Debug)]
pub enum LatticeDimensionBounds {
    /// No bounding, the lattice extends infinitely in both directions.
    ///
    /// Indexing into the lattice will return the value at the index given.
    Infinite,
    /// Length bounded, the lattice extends through the range `[0, n)`.
    ///
    /// Indexing into the lattice at `n` will return the value at 0.
    ///
    /// If `n` is negative, indexing at 1 will loop around and return the value at `n+1`.
    ///
    /// ## Example
    ///
    /// if `n` is -2, indexing at 1 will return the value in the lattice at index -1.
    LengthBounded(i32),
    /// Range bounded, the lattice extends through the range `[a, b]`.
    ///
    /// Indexing into the lattice at `b+1` will return the value at index `a`, and indexing into
    /// the lattice at `a-'` will return the value at `b`.
    ///
    /// **N.B.**
    /// ```rust
    /// # use rust_intonation::lattice::dimension_bounds::LatticeDimensionBounds::RangeBounded;
    /// RangeBounded(0, 2);
    /// ```
    /// is *not* the same as
    /// ```rust
    /// # use rust_intonation::lattice::dimension_bounds::LatticeDimensionBounds::LengthBounded;
    /// LengthBounded(2);
    /// ```
    ///
    /// The first will have a total length of 3 (since the range bound is inclusive),
    /// while the second will have a length of 2.
    RangeBounded(i32, i32),
}

impl LatticeDimensionBounds {
    pub fn resolve_index(&self, index: i32) -> i32 {
        match self {
            Self::Infinite => index,
            Self::LengthBounded(n) => sign_preserving_mod(index, *n),
            Self::RangeBounded(a, b) => {
                let modulo = b - a + 1;
                let abs_a = a.abs();

                sign_preserving_mod(index + abs_a, modulo) - abs_a
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LatticeDimensionBounds::*;

    #[test]
    fn resolve_index_for_infinite_bounds() {
        let bounds = Infinite;

        assert_eq!(bounds.resolve_index(0), 0);
        assert_eq!(bounds.resolve_index(1), 1);
        assert_eq!(bounds.resolve_index(-42), -42);
        assert_eq!(bounds.resolve_index(103), 103);
    }

    #[test]
    fn resolve_index_for_length_bounded() {
        let bounds = LengthBounded(2);

        assert_eq!(bounds.resolve_index(0), 0);
        assert_eq!(bounds.resolve_index(1), 1);
        assert_eq!(bounds.resolve_index(2), 0);
        assert_eq!(bounds.resolve_index(-42), 0);
        assert_eq!(bounds.resolve_index(103), 1);
    }

    #[test]
    fn resolve_index_for_range_bounded() {
        let bounds = RangeBounded(-2, 3);

        assert_eq!(bounds.resolve_index(0), 0);
        assert_eq!(bounds.resolve_index(1), 1);
        assert_eq!(bounds.resolve_index(2), 2);
        assert_eq!(bounds.resolve_index(4), -2);
        assert_eq!(bounds.resolve_index(-3), 3);
    }
}
