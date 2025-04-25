// Last updated: 25.04.2025, 15:35:00
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut rows = HashMap::with_capacity(grid.len());
        for row in &grid {
            rows.entry(row).and_modify(|count| *count += 1).or_insert(1);
        }
        let mut column = vec![0; grid.len()];
        let mut pairs = 0;
        for j in 0..grid.len() {
            for (i, cell) in column.iter_mut().enumerate() {
                *cell = grid[i][j];
            }
            pairs += *rows.get(&column).unwrap_or(&0);
        }
        pairs
    }
}