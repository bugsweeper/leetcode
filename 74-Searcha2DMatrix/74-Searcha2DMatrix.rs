// Last updated: 05.05.2025, 23:18:03
use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut base = 0;
        let mut size = m * n;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            match matrix[middle / n][middle % n].cmp(&target) {
                Ordering::Less => base = middle,
                Ordering::Equal => return true,
                _ => {}
            }
            size -= half;
        }
        matrix[base / n][base % n] == target
    }
}