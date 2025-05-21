// Last updated: 21.05.2025, 10:26:04
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.first().unwrap().len();
        let mut marker_row_index = usize::MAX;
        let mut marker_column_index = usize::MAX;
        'search: for (row_index, row) in matrix.iter().enumerate() {
            for (column_index, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    marker_row_index = row_index;
                    marker_column_index = column_index;
                    break 'search;
                }
            }
        }
        // could be no zeros
        if marker_row_index == usize::MAX {
            return;
        }
        // mark all rows and column
        let (with_marker, checked) = matrix.split_at_mut(marker_row_index + 1);
        let marker_row = with_marker.last_mut().unwrap();
        for checked_row in checked {
            for j in 0..n {
                if checked_row[j] == 0 {
                    checked_row[marker_column_index] = 0;
                    marker_row[j] = 0;
                }
            }
        }
        // fill rows
        for (i, row) in matrix.iter_mut().enumerate() {
            // except marker row
            if i == marker_row_index {
                continue;
            }
            if row[marker_column_index] != 0 {
                continue;
            }
            for cell in row {
                *cell = 0;
            }
        }
        // fill columns
        for j in 0..n {
            // except marker column
            if j == marker_column_index {
                continue;
            }
            if matrix[marker_row_index][j] != 0 {
                continue;
            }
            for row in matrix.iter_mut() {
                row[j] = 0;
            }
        }
        // fill marker row
        let row = &mut matrix[marker_row_index];
        for cell in row {
            *cell = 0;
        }
        // fill marker column
        for row in matrix {
            row[marker_column_index] = 0;
        }
    }
}