// Last updated: 20.08.2025, 16:26:13
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let width = matrix[0].len();
        let mut prev_row_square_size = vec![0; width];
        let mut square_size = vec![0; width];
        let mut squares = 0;
        for row in matrix {
            square_size[0] = if row[0] == 1 {
                squares += 1;
                1
            } else {
                0
            };
            for (column, cell) in row.into_iter().enumerate().skip(1) {
                square_size[column] = if cell == 1 {
                    let cur_square_size = 1 + square_size[column - 1]
                        .min(prev_row_square_size[column])
                        .min(prev_row_square_size[column - 1]);
                    squares += cur_square_size;
                    cur_square_size
                } else {
                    0
                }
            }
            std::mem::swap(&mut prev_row_square_size, &mut square_size);
        }
        squares
    }
}
