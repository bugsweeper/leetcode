// Last updated: 15.05.2025, 14:54:42
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut rook_i, mut rook_j) = (0, 0);
        'board: for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 'R' {
                    (rook_i, rook_j) = (i, j);
                    break 'board;
                }
            }
        }
        let mut pawns = 0;
        for i in rook_i + 1..8 {
            match board[i][rook_j] {
                'p' => {
                    pawns += 1;
                    break;
                }
                'B' => break,
                _ => {},
            }
        }
        for i in (0..rook_i).rev() {
            match board[i][rook_j] {
                'p' => {
                    pawns += 1;
                    break;
                }
                'B' => break,
                _ => {},
            }
        }
        let row = &board[rook_i];
        for j in rook_j + 1..8 {
            match row[j] {
                'p' => {
                    pawns += 1;
                    break;
                }
                'B' => break,
                _ => {},
            }
        }
        for j in (0..rook_j).rev() {
            match row[j] {
                'p' => {
                    pawns += 1;
                    break;
                }
                'B' => break,
                _ => {},
            }
        }
        pawns
    }
}