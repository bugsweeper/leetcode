// Last updated: 07.05.2025, 13:15:57
use std::collections::BinaryHeap;

#[inline]
fn try_in_matrix(
    pending: &mut [Vec<bool>],
    time: i32,
    move_time: &[Vec<i32>],
    i: usize,
    j: usize,
    queue: &mut BinaryHeap<(i32, usize, usize)>,
) {
    try_in_row(
        pending.get_mut(i).unwrap(),
        time,
        move_time.get(i).unwrap(),
        i,
        j,
        queue,
    );
}

#[inline]
fn try_in_row(
    pending: &mut [bool],
    time: i32,
    move_time: &[i32],
    i: usize,
    j: usize,
    queue: &mut BinaryHeap<(i32, usize, usize)>,
) {
    let pending = pending.get_mut(j).unwrap();
    if *pending {
        let next_time = time.min(-move_time[j]) - 1;
        *pending = false;
        queue.push((next_time, i, j));
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let mut queue = BinaryHeap::with_capacity(100);
        let m = move_time.len();
        let n = move_time[0].len();
        let (max_i, max_j) = (m - 1, n - 1);
        let mut pending = vec![vec![true; n]; m];
        pending[0][0] = false;
        queue.push((0, 0, 0));
        while let Some((time, i, j)) = queue.pop() {
            if i == max_i && j == max_j {
                return -time;
            }
            if i > 0 {
                try_in_matrix(&mut pending, time, &move_time, i - 1, j, &mut queue);
            }
            if i < max_i {
                try_in_matrix(&mut pending, time, &move_time, i + 1, j, &mut queue);
            }
            let (pending_row, move_time_row) = (pending.get_mut(i).unwrap(), move_time.get(i).unwrap());
            if j > 0 {
                try_in_row(pending_row, time, move_time_row, i, j - 1, &mut queue);
            }
            if j < max_j {
                try_in_row(pending_row, time, move_time_row, i, j + 1, &mut queue);
            }
        }
        0
    }
}