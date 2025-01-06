impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
      let mut seen_column = [[false;9];9];
      let mut seen_square = [[false;9];9];
      for (row_index, row) in board.iter().enumerate() {
        let mut seen_row = [false;9];
        for (column_index, &cell) in row.iter().enumerate() {
          let mut digit = (cell as u8) & 0x0f;
          if digit < 10 {
            digit -= 1;
            let seen_in_row = unsafe{seen_row.get_unchecked_mut(digit as usize)};
            if *seen_in_row {
              return false;
            }
            let seen_in_column = unsafe{seen_column.get_unchecked_mut(column_index).get_unchecked_mut(digit as usize)};
            if *seen_in_column {
              return false;
            }
            let seen_in_square = unsafe{seen_square.get_unchecked_mut(column_index / 3 + row_index / 3 * 3).get_unchecked_mut(digit as usize)};
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