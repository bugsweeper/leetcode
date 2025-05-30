// Last updated: 30.05.2025, 13:15:52
macro_rules! process {
    ( $value:expr, $i:expr, $j:expr, $minutes:ident, $max_minutes:ident, $remaining_oranges:ident, $queue:ident ) => {{
        let cell = $value;
        if *cell == 1 {
            *cell = 2;
            $remaining_oranges -= 1;
            let next_minutes = $minutes + 1;
            if $remaining_oranges == 0 {
                return next_minutes;
            }
            $queue.push_back(($i, $j, next_minutes));
            $max_minutes = $max_minutes.max(next_minutes);
        }
    }};
}

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut queue = VecDeque::with_capacity(m * n);
        let mut remaining_oranges = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match *cell {
                    1 => remaining_oranges += 1,
                    2 => queue.push_back((i, j, 0)),
                    _ => (),
                }
            }
        }
        if remaining_oranges == 0 {
            return 0;
        }
        let mut max_minutes = 0;
        let max_i = m as usize - 1;
        let max_j = n as usize - 1;
        while let Some((i, j, minutes)) = queue.pop_front() {
            if i > 0 {
                process!(
                    &mut grid[i - 1][j],
                    i - 1,
                    j,
                    minutes,
                    max_minutes,
                    remaining_oranges,
                    queue
                );
            }
            if i < max_i {
                process!(
                    &mut grid[i + 1][j],
                    i + 1,
                    j,
                    minutes,
                    max_minutes,
                    remaining_oranges,
                    queue
                );
            }
            let row = &mut grid[i];
            if j > 0 {
                process!(
                    &mut row[j - 1],
                    i,
                    j - 1,
                    minutes,
                    max_minutes,
                    remaining_oranges,
                    queue
                );
            }
            if j < max_j {
                process!(
                    &mut row[j + 1],
                    i,
                    j + 1,
                    minutes,
                    max_minutes,
                    remaining_oranges,
                    queue
                );
            }
        }
        -1
    }
}