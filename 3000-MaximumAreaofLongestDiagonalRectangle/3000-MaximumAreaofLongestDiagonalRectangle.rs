// Last updated: 26.08.2025, 09:46:46
use std::cmp::Ordering;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diagonal_square = 0;
        let mut max_area = 0;
        for dimension in dimensions {
            let (a, b) = (dimension[0], dimension[1]);
            let diagonal_square = a * a + b * b;
            match diagonal_square.cmp(&max_diagonal_square) {
                Ordering::Greater => {
                    max_diagonal_square = diagonal_square;
                    max_area = a * b;
                },
                Ordering::Equal => max_area = max_area.max(a * b),
                _ => {}
            }
        }
        max_area
    }
}