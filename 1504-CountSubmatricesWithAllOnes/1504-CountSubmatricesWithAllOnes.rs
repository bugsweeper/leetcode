// Last updated: 21.08.2025, 22:21:07
impl Solution {
    pub fn num_submat(matrix: Vec<Vec<i32>>) -> i32 {
        let width = matrix[0].len();
        let mut column_height = vec![0; width];
        let mut monotonic_stack: Vec<(usize, usize, usize)> = Vec::with_capacity(width + 1);
        let mut rectangles = 0;
        for row in matrix {
            monotonic_stack.clear();
            let mut ones_start_from = 0;
            for (column, (cell, cur_column_height)) in
                row.into_iter().zip(&mut column_height).enumerate()
            {
                if cell == 1 {
                    let (mut lower_index, mut lower_rectangles) = (ones_start_from, 0);
                    *cur_column_height += 1;
                    while let Some((index, rectangles, column_height)) = monotonic_stack.pop() {
                        if column_height < *cur_column_height {
                            lower_index = index;
                            lower_rectangles = rectangles;
                            monotonic_stack.push((index, rectangles, column_height));
                            break;
                        }
                    }
                    let new_rectangles =
                        lower_rectangles + *cur_column_height * (column + 1 - lower_index);
                    monotonic_stack.push((column + 1, new_rectangles, *cur_column_height));
                    rectangles += new_rectangles;
                } else {
                    monotonic_stack.clear();
                    ones_start_from = column + 1;
                    *cur_column_height = 0;
                }
            }
        }
        rectangles as i32
    }
}