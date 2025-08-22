// Last updated: 22.08.2025, 10:25:37
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut top, mut bottom, width, mut right) = (grid.len(), 0, grid[0].len(), grid.len());
        let mut left = width;
        let max_right = width - 1;
        let index_of_ones =
            |(index, cell): (usize, &i32)| if *cell == 1 { Some(index) } else { None };
        for (index, row) in grid.iter().enumerate() {
            let mut ones_iter = row.iter().enumerate().filter_map(index_of_ones);
            if let Some(column_index) = ones_iter.next() {
                top = index;
                bottom = index;
                left = column_index;
                right = max_right - ones_iter.next_back().unwrap_or(column_index);
                break;
            }
        }
        if left == width {
            return 0;
        }
        for (index, row) in grid.iter().enumerate().skip(top + 1).rev() {
            let mut ones_iter = row.iter().enumerate().filter_map(index_of_ones);
            if let Some(column_index) = ones_iter.next() {
                bottom = index;
                left = left.min(column_index);
                right = right.min(max_right - ones_iter.next_back().unwrap_or(column_index));
                break;
            }
        }
        if left == 0 && right == 0 {
            return ((bottom - top + 1) * width) as i32
        }
        let is_one = |cell: &i32| *cell == 1;
        for row in grid.into_iter().take(bottom).skip(top + 1) {
            if let Some(new_left) = row.iter().take(left).position(is_one) {
                left = new_left;
            }
            if let Some(new_right) = row.iter().rev().take(right).position(is_one) {
                right = new_right;
            }
        }
        ((bottom - top + 1) * (width - left - right)) as i32
    }
}
