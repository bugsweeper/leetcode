impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .map(|row| row.len() - row.partition_point(|&item| item >= 0))
            .sum::<usize>() as i32
    }
}