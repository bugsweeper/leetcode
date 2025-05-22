// Last updated: 22.05.2025, 14:04:18
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let mn = m * n;
        let mut iter = grid.into_iter().flatten().cycle().skip(mn - k as usize % mn).take(mn);
        let mut result = vec![vec![0; n]; m];
        for row in &mut result {
            for cell in row {
                *cell = iter.next().unwrap();
            }
        }
        result
    }
}