use num::PrimInt;

use crate::ratio::Ratio;
use std::{fmt::Display, marker::PhantomData};

pub struct Diamond<T: PrimInt = i32> {
    pub identities: Vec<u32>,
    phantom: PhantomData<T>,
}

impl<T: PrimInt> Display for Diamond<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ratios = self.generate();
        let output = self
            .index_coordinates()
            .iter()
            .map(|row| self.construct_diamond_row(row, &ratios))
            .collect::<Vec<String>>()
            .join("\n\n");
        write!(f, "{}", output)
    }
}

type Coordinate = (usize, usize);
type Coordinates = Vec<Coordinate>;

impl<T: PrimInt> Diamond<T> {
    pub fn new(identities: Vec<u32>) -> Self {
        Self {
            identities,
            phantom: PhantomData::<T>,
        }
    }

    pub fn generate(&self) -> Vec<Vec<Ratio<i32>>> {
        self.identities
            .iter()
            .map(|d| self.construct_ratios_with_denominator(*d as i32))
            .collect()
    }

    fn construct_ratios_with_denominator(&self, denominator: i32) -> Vec<Ratio<i32>> {
        self.identities
            .iter()
            .map(|n| Ratio::new(*n as i32, denominator).normalize())
            .collect()
    }

    fn construct_diamond_row(&self, row: &[Coordinate], ratios: &[Vec<Ratio<i32>>]) -> String {
        let prefix_len = self.identities.len() - row.len();
        let prefix = "\t".repeat(prefix_len);
        format!(
            "{}{}",
            prefix,
            row.iter()
                .map(|(a, b)| format!("{}", &ratios[*a][*b]))
                .collect::<Vec<String>>()
                .join("\t\t")
        )
    }

    fn index_coordinates(&self) -> Vec<Coordinates> {
        let max = self.identities.len() - 1;
        let mut coordinate_rows = vec![];
        for i in (0..=max).rev() {
            let row: Coordinates = (i..=max).enumerate().collect();
            coordinate_rows.push(row);
        }
        for i in 1..=max {
            let row: Coordinates = (i..=max).enumerate().map(|(a, b)| (b, a)).collect();
            coordinate_rows.push(row);
        }

        coordinate_rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn five_limit() {
        let d: Diamond = Diamond::new(vec![1, 3, 5]);
        let g = d.generate();

        assert_eq!(g[0][0], Ratio::new(1, 1));
        assert_eq!(g[0][1], Ratio::new(3, 2));
        assert_eq!(g[0][2], Ratio::new(5, 4));

        assert_eq!(g[1][0], Ratio::new(4, 3));
        assert_eq!(g[1][1], Ratio::new(1, 1));
        assert_eq!(g[1][2], Ratio::new(5, 3));

        assert_eq!(g[2][0], Ratio::new(8, 5));
        assert_eq!(g[2][1], Ratio::new(6, 5));
        assert_eq!(g[2][2], Ratio::new(1, 1));
    }
}
