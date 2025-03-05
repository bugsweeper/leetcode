impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * n - 2 * n + 1
    }
}