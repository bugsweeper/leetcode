use std::collections::VecDeque;

#[inline]
fn process_row(queue: &mut VecDeque<(usize, usize)>, row: &mut [i32], i: usize, j: usize) -> i32 {
    let fishes_mut = unsafe { row.get_unchecked_mut(j) };
    let fishes = *fishes_mut;
    if fishes == 0 {
        return 0;
    }
    *fishes_mut = 0;
    queue.push_back((i, j));
    fishes
}

#[inline]
fn process_grid(
    queue: &mut VecDeque<(usize, usize)>,
    grid: &mut [Vec<i32>],
    i: usize,
    j: usize,
) -> i32 {
    process_row(queue, unsafe { grid.get_unchecked_mut(i) }, i, j)
}

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = unsafe { grid.get_unchecked(0) }.len();
        let mut grid = grid;
        let mut max_fishes = 0;
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                let mut current_fishes = process_grid(&mut queue, &mut grid, i, j);
                while let Some((i, j)) = queue.pop_front() {
                    if i > 0 {
                        current_fishes += process_grid(&mut queue, &mut grid, i - 1, j);
                    }
                    if i < m - 1 {
                        current_fishes += process_grid(&mut queue, &mut grid, i + 1, j);
                    }
                    let row = unsafe { grid.get_unchecked_mut(i) };
                    if j > 0 {
                        current_fishes += process_row(&mut queue, row, i, j - 1);
                    }
                    if j < n - 1 {
                        current_fishes += process_row(&mut queue, row, i, j + 1);
                    }
                }
                max_fishes = max_fishes.max(current_fishes);
            }
        }
        max_fishes
    }
}