// Last updated: 11.08.2025, 15:24:03
impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        for (r, row) in grid.into_iter().enumerate() {
            let opposite = row.len() - r - 1;
            for (c, cell) in row.into_iter().enumerate() {
                if (c == r || c == opposite) == (cell == 0) {
                    return false;
                }
            }
        }
        true
    }
}