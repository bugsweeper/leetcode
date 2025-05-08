// Last updated: 08.05.2025, 14:05:15
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut prev_row, mut grid) = grid.split_first().unwrap();
        let mut surface = 0;
        let mut left_cell = 0;
        for &cell in prev_row {
            surface += if cell == 0 { 0 } else { 1 } + (cell << 1) - left_cell.min(cell);
            left_cell = cell;
        }
        while let Some((row, remaining)) = grid.split_first() {
            left_cell = 0;
            for (&upper_cell, &cell) in prev_row.iter().zip(row) {
                surface += if cell == 0 { 0 } else { 1 } + (cell << 1) - left_cell.min(cell) - upper_cell.min(cell);
                left_cell = cell;
            }
            (prev_row, grid) = (row, remaining);
        }
        surface << 1
    }
}