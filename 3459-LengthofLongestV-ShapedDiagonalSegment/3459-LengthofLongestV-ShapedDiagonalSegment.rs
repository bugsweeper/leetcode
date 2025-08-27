// Last updated: 28.08.2025, 00:19:13
use std::collections::HashMap;

const SHIFT: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn recursive_search(
    grid: &[Vec<i32>],
    dp: &mut HashMap<(usize, usize, usize, bool), i32>,
    i: usize,
    j: usize,
    direction: usize,
    from_two: bool,
    made_turn: bool,
) -> i32 {
    match (grid[i][j], from_two) {
        (0, true) | (2, false) => {}
        _ => return 0,
    }
    if let Some(&len) = dp.get(&(i, j, direction, made_turn)) {
        return len;
    }
    let shifts = if made_turn {
        vec![(direction, true)]
    } else {
        vec![(direction, false), ((direction + 1) % 4, true)]
    };
    let mut max_len = 0;
    for (direction, made_turn) in shifts {
        let (shift_i, shift_j) = SHIFT[direction];
        if let (Some(next_i), Some(next_j)) =
            (i.checked_add_signed(shift_i), j.checked_add_signed(shift_j))
            && next_i < grid.len()
            && next_j < grid[0].len()
        {
            max_len = max_len.max(recursive_search(
                grid, dp, next_i, next_j, direction, !from_two, made_turn,
            ));
        }
    }
    max_len += 1;
    dp.insert((i, j, direction, made_turn), max_len);
    max_len
}

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = HashMap::with_capacity(grid.len() * grid[0].len() * 8);
        let max_i = grid.len() - 1;
        let max_j = grid[0].len() - 1;
        let mut max_len = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell != 1 {
                    continue;
                }
                let mut len = 0;
                if i > 0 {
                    if j > 0 {
                        len = len.max(recursive_search(
                            &grid,
                            &mut dp,
                            i - 1,
                            j - 1,
                            2,
                            false,
                            false,
                        ));
                    }
                    if j < max_j {
                        len = len.max(recursive_search(
                            &grid,
                            &mut dp,
                            i - 1,
                            j + 1,
                            3,
                            false,
                            false,
                        ))
                    }
                }
                if i < max_i {
                    if j > 0 {
                        len = len.max(recursive_search(
                            &grid,
                            &mut dp,
                            i + 1,
                            j - 1,
                            1,
                            false,
                            false,
                        ));
                    }
                    if j < max_j {
                        len = len.max(recursive_search(
                            &grid,
                            &mut dp,
                            i + 1,
                            j + 1,
                            0,
                            false,
                            false,
                        ))
                    }
                }
                max_len = max_len.max(len + 1);
            }
        }
        max_len
    }
}