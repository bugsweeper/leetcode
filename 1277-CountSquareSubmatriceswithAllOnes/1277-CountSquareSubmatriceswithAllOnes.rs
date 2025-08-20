// Last updated: 20.08.2025, 16:12:09
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let width = matrix[0].len();
        let mut column_height = vec![0; width];
        let mut prev_row_square_size = vec![0; width];
        let mut square_size = vec![0; width];
        let mut squares = 0;
        for row in matrix {
            let mut row_width = 0;
            for (column, (cell, column_height)) in
                row.into_iter().zip(&mut column_height).enumerate()
            {
                if cell == 1 {
                    let cur_square_size = if column == 0 {
                        1
                    } else {
                        1 + row_width
                            .min(*column_height)
                            .min(prev_row_square_size[column - 1])
                    };
                    row_width += 1;
                    *column_height += 1;
                    square_size[column] = cur_square_size;
                    squares += cur_square_size;
                } else {
                    row_width = 0;
                    *column_height = 0;
                    square_size[column] = 0;
                }
            }
            std::mem::swap(&mut prev_row_square_size, &mut square_size);
        }
        squares
    }
}