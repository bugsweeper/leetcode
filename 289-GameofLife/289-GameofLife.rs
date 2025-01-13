#[inline]
fn calc_row(row: &[i32], j: usize, middle: bool) -> i32 {
    let mut sum = 0;
    if j > 0 {
        sum += unsafe { row.get_unchecked(j - 1) };
    }
    if middle {
        sum += unsafe { row.get_unchecked(j) };
    }
    if j < row.len() - 1 {
        sum += unsafe { row.get_unchecked(j + 1) };
    }
    sum
}

#[inline]
fn calc_value(board: &[Vec<i32>], i: usize, j: usize) -> i32 {
    let mut sum = 0;
    if i > 0 {
        sum += calc_row(unsafe { board.get_unchecked(i - 1) }, j, true);
    }
    sum += calc_row(unsafe { board.get_unchecked(i) }, j, false);
    if i < board.len() - 1 {
        sum += calc_row(unsafe { board.get_unchecked(i + 1) }, j, true);
    }
    match (sum, *unsafe { board.get_unchecked(i).get_unchecked(j) }) {
        (2, 1) | (3, _) => 1,
        _ => 0,
    }
}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = unsafe { board.get_unchecked(0) }.len();
        let mut next_upper = vec![0; n];
        let mut next_middle = vec![0; n];
        for i in 0..m {
            // Calculate current row
            for j in 0..n {
                *unsafe { next_middle.get_unchecked_mut(j) } = calc_value(board, i, j);
            }
            // Upper row is not needed, replace with previously calculated upper row
            if i > 0 {
                std::mem::swap(&mut next_upper, unsafe { board.get_unchecked_mut(i - 1) });
            }
            // Prepare current row as upper row for next iteration
            std::mem::swap(&mut next_upper, &mut next_middle);
        }
        std::mem::swap(&mut next_upper, unsafe { board.get_unchecked_mut(m - 1) });
    }
}