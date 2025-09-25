// Last updated: 25.09.2025, 06:01:34
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut min_sum_row = vec![0; triangle.len()];
        for row in triangle {
            if row.len() > 1 {
                min_sum_row[row.len() - 1] = min_sum_row[row.len() - 2] + *row.last().unwrap();
            }
            let first = *row.first().unwrap();
            for (i, cell) in row.into_iter().enumerate().skip(1).rev().skip(1) {
                min_sum_row[i] = min_sum_row[i].min(min_sum_row[i - 1]) + cell;
            }
            min_sum_row[0] += first;
        }
        min_sum_row.into_iter().min().unwrap()
    }
}