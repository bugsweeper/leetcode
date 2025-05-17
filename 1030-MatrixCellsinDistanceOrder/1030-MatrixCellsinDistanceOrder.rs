// Last updated: 17.05.2025, 22:18:54
impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity((rows * cols) as usize);
        result.push(vec![r_center, c_center]);
        let (mut r, mut c);
        for distance in 1..=(r_center.max(rows - r_center - 1) + c_center.max(cols - c_center - 1)) {
            c = c_center + distance;
            for step in 0.max(c - cols + 1)..distance.min(r_center + 1) {
                result.push(vec![r_center - step, c - step]);
            }
            r = r_center - distance;
            for step in 0.max(-r)..distance.min(c_center + 1) {
                result.push(vec![r + step, c_center - step]);
            }
            c = c_center - distance;
            for step in 0.max(-c)..distance.min(rows - r_center) {
                result.push(vec![r_center + step, c + step]);
            }
            r = r_center + distance;
            for step in 0.max(r - rows + 1)..distance.min(cols - c_center) {
                result.push(vec![r - step, c_center + step]);
            }
        }
        result
    }
}