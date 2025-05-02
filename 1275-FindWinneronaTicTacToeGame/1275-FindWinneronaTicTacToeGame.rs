// Last updated: 02.05.2025, 10:40:47
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = [[' '; 3]; 3];
        let mut player = 'A';
        for coord in &moves {
            let [i, j] = coord[..] else { unimplemented!() };
            board[i as usize][j as usize] = player;
            player = if player == 'A' { 'B' } else { 'A' };
        }
        for row in board {
            if row[0] != ' ' && row[0] == row[1] && row[0] == row[2] {
                return row[0].into();
            }
        }
        for j in 0..3 {
            if board[0][j] != ' ' && board[0][j] == board[1][j] && board[0][j] == board[2][j] {
                return board[0][j].into();
            }
        }
        if board[0][0] != ' ' && board[0][0] == board[1][1] && board[0][0] == board[2][2] {
            return board[0][0].into();
        }
        if board[0][2] != ' ' && board[0][2] == board[1][1] && board[0][2] == board[2][0] {
            return board[0][2].into();
        }
        if moves.len() == 9 {
            "Draw".into()
        } else {
            "Pending".into()
        }
    }
}