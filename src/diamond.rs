use crate::ratio::Ratio;

pub struct Diamond {
    pub identities: Vec<u32>,
}

impl Diamond {
    pub fn new(identities: Vec<u32>) -> Self {
        Self { identities }
    }

    pub fn generate(&self) -> Vec<Vec<Ratio>> {
        self.identities
            .iter()
            .map(|d| {
                self.identities
                    .iter()
                    .map(|n| Ratio::new(*n as i32, *d as i32).normalize())
                    .collect::<Vec<Ratio>>()
            })
            .collect::<Vec<Vec<Ratio>>>()
    }

    pub fn display(&self) -> String {
        let ratios = self.generate();
        self.index_coordinates()
            .iter()
            .map(|row| self.construct_lattice_row(row, &ratios))
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    fn construct_lattice_row(&self, row: &[(usize, usize)], ratios: &[Vec<Ratio>]) -> String {
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

    fn index_coordinates(&self) -> Vec<Vec<(usize, usize)>> {
        let max = self.identities.len() - 1;
        let mut coordinate_rows = vec![];
        for i in (0..=max).rev() {
            let row = (i..=max).enumerate().collect::<Vec<(usize, usize)>>();
            coordinate_rows.push(row);
        }
        for i in 1..=max {
            let row = (i..=max)
                .enumerate()
                .map(|(a, b)| (b, a))
                .collect::<Vec<(usize, usize)>>();
            coordinate_rows.push(row);
        }

        coordinate_rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_limit() {
        let d = Diamond::new(vec![1, 3, 5]);
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
