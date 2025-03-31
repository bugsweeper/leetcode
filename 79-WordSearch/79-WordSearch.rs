// Last updated: 31.03.2025, 14:55:25
fn continues(board: &Vec<Vec<char>>, word: &Vec<char>, seen: &mut [bool; 36], i: usize, j: usize, mut index: usize) -> bool {
    let row = board.get(i).unwrap();
    if row[j] != word[index] {
        return false;
    }
    if index == word.len() - 1 {
        return true;
    }
    let seen_index = i * row.len() + j;
    seen[seen_index] = true;
    index += 1;
    if i > 0 && !seen[seen_index - row.len()] && continues(board, word, seen, i - 1, j, index) {
        return true;
    }
    if i < board.len() - 1 && !seen[seen_index + row.len()] && continues(board, word, seen, i + 1, j, index) {
        return true;
    }

    if j > 0 && !seen[seen_index - 1] && continues(board, word, seen, i, j - 1, index) {
        return true;
    }
    if j < row.len() - 1 && !seen[seen_index + 1] && continues(board, word, seen, i, j + 1, index) {
        return true;
    }
    seen[i * row.len() + j] = false;
    false
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 1 {
            let first = word.as_bytes()[0] as char;
            return board.iter().any(|row| row.contains(&first));
        }
        let m = board.len();
        let n = board[0].len();
        let word = word
            .as_bytes()
            .iter()
            .map(|&byte| byte as char)
            .collect::<Vec<_>>();
        let mut seen = [false; 36];
        for i in 0..m {
            for j in 0..n {
                if continues(&board, &word, &mut seen, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
}