// Last updated: 28.08.2025, 11:03:53
impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let last = n - 1;
        let mut grid = grid;
        let mut diagonal = Vec::with_capacity(n);
        for diagonal_index in 0..2 * n - 1 {
            let first_row = diagonal_index.saturating_sub(last);
            let last_row = diagonal_index.min(last);
            let first_col = (first_row + last).saturating_sub(diagonal_index);
            diagonal.clear();
            for (i, j) in (first_row..=last_row).zip(first_col..) {
                diagonal.push(grid[i][j]);
            }
            if diagonal_index < last {
                diagonal.sort_unstable();
            } else {
                diagonal.sort_unstable_by_key(|&num| -num);
            }
            for (&num, (i, j)) in diagonal.iter().zip((first_row..=last_row).zip(first_col..)) {
                grid[i][j] = num;
            }
        }
        grid
    }
}