// Last updated: 08.05.2025, 10:29:34
use std::collections::BinaryHeap;

#[inline]
fn try_vertical_move(
    i: usize,
    j: usize,
    time: i32,
    add: usize,
    time_matrix: &[Vec<i32>],
    pending_matrix: &mut [Vec<bool>],
    queue: &mut BinaryHeap<(i32, usize, usize)>,
) {
    let pending = pending_matrix[i].get_mut(j).unwrap();
    if *pending {
        let next_time = time.min(-time_matrix[i][j]) - add as i32;
        queue.push((next_time, i, j));
        *pending = false;
    }
}

#[inline]
fn try_horizontal_move(
    i: usize,
    j: usize,
    time: i32,
    add: usize,
    time_row: &[i32],
    pending_row: &mut [bool],
    queue: &mut BinaryHeap<(i32, usize, usize)>,
) {
    let pending = pending_row.get_mut(j).unwrap();
    if *pending {
        let next_time = time.min(-time_row[j]) - add as i32;
        queue.push((next_time, i, j));
        *pending = false;
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let mut queue = BinaryHeap::with_capacity(m + n);
        let min_i = n - 1;
        let min_j = m - 1;
        queue.push((0, 0, 0));
        let mut pending = vec![vec![true; m]; n];
        pending[0][0] = false;
        while let Some((time, i, j)) = queue.pop() {
            if i == min_i && j == min_j {
                return -time;
            }
            let add = 1 + (i + j) % 2;
            if i > 0 {
                try_vertical_move(i - 1, j, time, add, &move_time, &mut pending, &mut queue);
            }
            if i < min_i {
                try_vertical_move(i + 1, j, time, add, &move_time, &mut pending, &mut queue);
            }
            let (time_row, pending_row) = (move_time.get(i).unwrap(), pending.get_mut(i).unwrap());
            if j > 0 {
                try_horizontal_move(i, j - 1, time, add, time_row, pending_row, &mut queue);
            }
            if j < min_j {
                try_horizontal_move(i, j + 1, time, add, time_row, pending_row, &mut queue);
            }
        }
        0
    }
}