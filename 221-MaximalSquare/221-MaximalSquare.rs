// Last updated: 30.04.2025, 12:10:24
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (prev_matrix_row, mut matrix) = matrix.split_first().unwrap();
        let mut maximal_size = 0;
        let mut prev_squares_row = Vec::with_capacity(prev_matrix_row.len());
        for &digit in prev_matrix_row {
            if digit == '1' {
                prev_squares_row.push(1);
                maximal_size = 1;
            } else {
                prev_squares_row.push(0);
            }
        }
        while let Some((matrix_row, remaining)) = matrix.split_first() {
            let (mut left_size, mut left_top_size) = (0, 0);
            let mut squares_row = Vec::with_capacity(matrix_row.len());
            for (&digit, top_size) in matrix_row.iter().zip(prev_squares_row) {
                (left_size, left_top_size) = (
                    if digit == '1' {
                        top_size.min(left_size).min(left_top_size) + 1
                    } else {
                        0
                    },
                    top_size,
                );
                maximal_size = maximal_size.max(left_size);
                squares_row.push(left_size);
            }
            (prev_squares_row, matrix) = (squares_row, remaining);
        }
        maximal_size * maximal_size
    }
}