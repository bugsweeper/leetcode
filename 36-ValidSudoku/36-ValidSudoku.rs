// Last updated: 30.08.2025, 14:56:27
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
      let mut seen_column = [[false;10];9];
      let mut seen_square = [[false;10];9];
      for (row_index, row) in board.iter().enumerate() {
        let mut seen_row = [false;10];
        for (column_index, (seen_column, &cell)) in seen_column.iter_mut().zip(row).enumerate() {
          let digit = ((cell as u8) & 0x0f) as usize;
          if digit < 10 {
            let seen_in_row = &mut seen_row[digit];
            if *seen_in_row {
              return false;
            }
            let seen_in_column = &mut seen_column[digit];
            if *seen_in_column {
              return false;
            }
            let seen_in_square = &mut seen_square[column_index / 3 + row_index / 3 * 3][digit];
            if *seen_in_square {
              return false;
            }
            *seen_in_row = true;
            *seen_in_column = true;
            *seen_in_square = true;
          }
        }
      }
      true
    }
}