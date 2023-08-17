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
            .map(|x| {
                self.identities
                    .iter()
                    .map(|y| Ratio::new(*y as i32, *x as i32).normalize())
                    .collect::<Vec<Ratio>>()
            })
            .collect::<Vec<Vec<Ratio>>>()
    }

    pub fn display(&self) -> String {
        let ratios = self.generate();
        index_coordinates(self.identities.len() - 1)
            .iter()
            .map(|row| {
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
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

fn index_coordinates(max: usize) -> Vec<Vec<(usize, usize)>> {
    let mut coordinate_rows = vec![];
    for i in (0..=max).rev() {
        let row = (i..=max)
            .into_iter()
            .enumerate()
            .map(|(a, b)| (a, b))
            .collect::<Vec<(usize, usize)>>();
        coordinate_rows.push(row);
    }
    for i in 1..=max {
        let row = (i..=max)
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<Vec<(usize, usize)>>();
        coordinate_rows.push(row);
    }

    coordinate_rows
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
