impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = unsafe{matrix.get_unchecked(0).len()};
        let mut marker_row = usize::MAX;
        let mut marker_column = usize::MAX;
        'search: for (row_index, row) in matrix.iter().enumerate() {
            for (column_index, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    marker_row = row_index;
                    marker_column = column_index;
                    break 'search;
                }
            }
        }
        // could be no zeros
        if marker_row == usize::MAX {
            return;
        }
        // mark all rows and column
        for i in marker_row + 1..m {
            for j in 0..n {
                if unsafe{*matrix.get_unchecked(i).get_unchecked(j)} == 0 {
                    *unsafe{matrix.get_unchecked_mut(i).get_unchecked_mut(marker_column)} = 0;
                    *unsafe{matrix.get_unchecked_mut(marker_row).get_unchecked_mut(j)} = 0;
                }
            }
        }
        // fill rows
        for i in 0..m {
            // except marker row
            if i == marker_row {
                continue;
            }
            if unsafe{*matrix.get_unchecked(i).get_unchecked(marker_column)} != 0 {
                continue;
            }
            let row = unsafe{matrix.get_unchecked_mut(i)};
            for j in 0..n {
                *unsafe{row.get_unchecked_mut(j)} = 0;
            }
        }
        // fill columns
        for j in 0..n {
            // except marker column
            if j == marker_column {
                continue;
            }
            if unsafe{*matrix.get_unchecked(marker_row).get_unchecked(j)} != 0 {
                continue;
            }
            for i in 0..m {
                *unsafe{matrix.get_unchecked_mut(i).get_unchecked_mut(j)} = 0;
            }
        }
        // fill marker row
        {
            let row = unsafe{matrix.get_unchecked_mut(marker_row)};
            for j in 0..n {
                *unsafe{row.get_unchecked_mut(j)} = 0;
            }
        }
        // fill marker column
        for i in 0..m {
            *unsafe{matrix.get_unchecked_mut(i).get_unchecked_mut(marker_column)} = 0;
        }
    }
}