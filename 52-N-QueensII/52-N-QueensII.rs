// Last updated: 16.05.2025, 14:53:42
fn count_recursive(current_row: usize, queen_row: &mut [Option<usize>]) -> i32 {
    let mut count = 0;
    'column: for column in 0..queen_row.len() {
        if queen_row[column].is_some() {
            continue;
        }
        for row in 0..current_row {
            let step = current_row - row;
            if column >= step && queen_row[column - step] == Some(row) {
                continue 'column;
            }
            if column + step < queen_row.len() && queen_row[column + step] == Some(row) {
                continue 'column;
            }
        }
        if current_row == queen_row.len() - 1 {
            return 1;
        }
        queen_row[column] = Some(current_row);
        count += count_recursive(current_row + 1, queen_row);
        queen_row[column] = None;
    }
    count
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut queen_row = vec![None; n];
        count_recursive(0, &mut queen_row)
    }
}