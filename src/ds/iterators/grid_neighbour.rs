use std::iter::Iterator;

pub struct GridNeighbour {
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    offset: Vec<(i64, i64)>,
    offset_idx: usize,
}

impl GridNeighbour {
    pub fn new(x: usize, y: usize, n: usize, m: usize, diagonal_traversal: bool) -> Self {
        let mut offset = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        if diagonal_traversal {
            offset.append(&mut vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
        }
        Self {
            x,
            y,
            n,
            m,
            offset,
            offset_idx: 0,
        }
    }
}

impl Iterator for GridNeighbour {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset_idx < self.offset.len() {
            let x = self.x as i64 + self.offset[self.offset_idx].0;
            let y = self.y as i64 + self.offset[self.offset_idx].1;
            self.offset_idx += 1;
            if x >= 0 && x < self.n as i64 && y >= 0 && y < self.m as i64 {
                return Some((x as usize, y as usize));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_diagonal_traversal() {
        const N: usize = 4;
        const M: usize = 5;
        let cases = vec![
            ((0, 0), vec![(0, 1), (1, 0)], "left top"),
            ((3, 0), vec![(2, 0), (3, 1)], "left bottom"),
            ((0, 4), vec![(0, 3), (1, 4)], "right top"),
            ((3, 4), vec![(3, 3), (2, 4)], "right bottom"),
            ((1, 0), vec![(0, 0), (1, 1), (2, 0)], "left border"),
            ((1, 4), vec![(1, 3), (0, 4), (2, 4)], "right border"),
            ((0, 1), vec![(0, 0), (0, 2), (1, 1)], "top border"),
            ((3, 1), vec![(3, 0), (3, 2), (2, 1)], "bottom border"),
            ((1, 2), vec![(0, 2), (2, 2), (1, 1), (1, 3)], "in grid"),
        ];
        for (input, mut expected, case_type) in cases {
            let neighbours = GridNeighbour::new(input.0, input.1, N, M, false);
            let mut output = neighbours.collect::<Vec<_>>();

            output.sort();
            expected.sort();
            debug_assert_eq!(expected, output, "{}", case_type);
        }
    }

    #[test]
    fn test_diagonal_traversal() {
        const N: usize = 4;
        const M: usize = 5;
        let cases = vec![
            ((0, 0), vec![(0, 1), (1, 0), (1, 1)], "left top"),
            ((3, 0), vec![(2, 0), (3, 1), (2, 1)], "left bottom"),
            ((0, 4), vec![(0, 3), (1, 4), (1, 3)], "right top"),
            ((3, 4), vec![(3, 3), (2, 4), (2, 3)], "right bottom"),
            (
                (1, 0),
                vec![(0, 0), (1, 1), (2, 0), (0, 1), (2, 1)],
                "left border",
            ),
            (
                (1, 4),
                vec![(1, 3), (0, 4), (2, 4), (0, 3), (2, 3)],
                "right border",
            ),
            (
                (0, 1),
                vec![(0, 0), (0, 2), (1, 1), (1, 0), (1, 2)],
                "top border",
            ),
            (
                (3, 1),
                vec![(3, 0), (3, 2), (2, 1), (2, 0), (2, 2)],
                "bottom border",
            ),
            (
                (1, 2),
                vec![
                    (0, 2),
                    (2, 2),
                    (1, 1),
                    (1, 3),
                    (0, 1),
                    (0, 3),
                    (2, 1),
                    (2, 3),
                ],
                "in grid",
            ),
        ];
        for (input, mut expected, case_type) in cases {
            let neighbours = GridNeighbour::new(input.0, input.1, N, M, true);
            let mut output = neighbours.collect::<Vec<_>>();

            output.sort();
            expected.sort();
            debug_assert_eq!(expected, output, "{}", case_type);
        }
    }
}
