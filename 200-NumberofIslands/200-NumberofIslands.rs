const MAX_GRID_SIZE: usize = 300;

#[inline]
fn process_grid(i: usize, j: usize, grid: &mut [Vec<char>], stack: &mut Vec<(usize, usize)>) {
    process_row(i, j, unsafe { grid.get_unchecked_mut(i) }, stack);
}

#[inline]
fn process_row(i: usize, j: usize, row: &mut [char], stack: &mut Vec<(usize, usize)>) {
    let next = unsafe { row.get_unchecked_mut(j) };
    if *next == '1' {
        *next = '0';
        stack.push((i, j));
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut result = 0;
        let mut stack = Vec::with_capacity(MAX_GRID_SIZE * MAX_GRID_SIZE);
        for i in 0..m {
            for j in 0..n {
                let current = unsafe { grid.get_unchecked_mut(i).get_unchecked_mut(j) };
                if *current == '1' {
                    result += 1;
                    *current = '0';
                    stack.push((i, j));
                    while let Some((i, j)) = stack.pop() {
                        if i > 0 {
                            process_grid(i - 1, j, &mut grid, &mut stack);
                        }
                        if i < m - 1 {
                            process_grid(i + 1, j, &mut grid, &mut stack);
                        }
                        let row = unsafe { grid.get_unchecked_mut(i) };
                        if j > 0 {
                            process_row(i, j - 1, row, &mut stack);
                        }
                        if j < n - 1 {
                            process_row(i, j + 1, row, &mut stack);
                        }
                    }
                }
            }
        }
        result
    }
}