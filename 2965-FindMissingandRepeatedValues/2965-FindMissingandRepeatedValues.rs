impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut twice = 0;
        let mut missing = 0;
        let mut seen = std::collections::HashSet::with_capacity(n * n);
        for row in grid {
            for cell in row {
                if !seen.insert(cell) {
                    twice = cell;
                }
            }
        }
        for val in 1..=(n * n) as i32 {
            if !seen.contains(&val) {
                missing = val;
                break;
            }
        }
        vec![twice, missing]
    }
}