#[inline]
fn mark_unsurrounded(board: &mut [Vec<char>], stack: &mut Vec<(usize, usize)>) {
    while let Some((i, j)) = stack.pop() {
        if i > 0 {
            let cell = board[i - 1].get_mut(j).unwrap();
            if *cell == 'O' {
                *cell = 'U';
                stack.push((i - 1, j));
            }
        }
        if i < board.len() - 1 {
            let cell = board[i + 1].get_mut(j).unwrap();
            if *cell == 'O' {
                *cell = 'U';
                stack.push((i + 1, j));
            }
        }
        let row = board.get_mut(i).unwrap();
        if j > 0 {
            let cell = row.get_mut(j - 1).unwrap();
            if *cell == 'O' {
                *cell = 'U';
                stack.push((i, j - 1));
            }
        }
        if j < row.len() - 1 {
            let cell = row.get_mut(j + 1).unwrap();
            if *cell == 'O' {
                *cell = 'U';
                stack.push((i, j + 1));
            }
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        let mut stack = Vec::with_capacity(m * n);
        for i in [0, m - 1] {
            for j in 0..n {
                let cell = board[i].get_mut(j).unwrap();
                if *cell == 'O' {
                    *cell = 'U';
                    stack.push((i, j));
                    mark_unsurrounded(board, &mut stack);
                }
            }
        }
        if m > 2 {
            for j in [0, n - 1] {
                for i in 1..m - 1 {
                    let cell = board[i].get_mut(j).unwrap();
                    if *cell == 'O' {
                        *cell = 'U';
                        stack.push((i, j));
                        mark_unsurrounded(board, &mut stack);
                    }
                }
            }
        }
        for row in board {
            for cell in row {
                match *cell {
                    'U' => *cell = 'O',
                    'O' => *cell = 'X',
                    _ => {}
                }
            }
        }
    }
}