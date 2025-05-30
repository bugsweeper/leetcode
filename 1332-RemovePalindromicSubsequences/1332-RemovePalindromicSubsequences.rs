// Last updated: 30.05.2025, 16:50:36
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut columns = vec![i32::MIN; m];
        let mut rows = Vec::with_capacity(n);
        for row in &matrix {
            rows.push(*row.iter().min().unwrap());
            for (&cell, max_column) in row.iter().zip(columns.iter_mut()) {
                *max_column = (*max_column).max(cell);
            }
        }
        let mut result = Vec::with_capacity(n.min(m));
        for (row, min_row) in matrix.iter().zip(rows.into_iter()) {
            for (&cell, &max_column) in row.iter().zip(columns.iter()) {
                if cell == min_row && cell == max_column {
                    result.push(cell);
                }
            }
        }
        result
    }
}