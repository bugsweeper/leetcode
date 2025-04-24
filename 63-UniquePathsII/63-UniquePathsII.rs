// Last updated: 24.04.2025, 12:25:01
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut obstacle_grid = obstacle_grid;
        let (mut prev_row, mut grid) = obstacle_grid.split_first_mut().unwrap();
        let mut paths = 1;
        for cell in prev_row.iter_mut() {
            if *cell == 1 {
                paths = 0;
            }
            *cell = paths;
        }
        while let Some((cur_row, remaining)) = grid.split_first_mut() {
            paths = 0;
            for (prev, cur) in prev_row.iter().zip(cur_row.iter_mut()) {
                if *cur == 0 {
                    paths += *prev;
                } else {
                    paths = 0;
                }
                *cur = paths;
            }
            (prev_row, grid) = (cur_row, remaining);
        }
        paths
    }
}