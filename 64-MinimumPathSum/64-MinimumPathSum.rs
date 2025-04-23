// Last updated: 23.04.2025, 12:59:45
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (mut prev, mut grid) = grid.split_first_mut().unwrap();
        let mut sum = 0;
        for cell in prev.iter_mut() {
            sum += *cell;
            *cell = sum;
        }
        while let Some((cur, remaining)) = grid.split_first_mut() {
            sum = i32::MAX;
            for (prev, cur) in prev.iter_mut().zip(cur.iter_mut()) {
                sum = sum.min(*prev) + *cur;
                *cur = sum;
            }
            (prev, grid) = (cur, remaining);
        }
        *prev.last().unwrap()
    }
}