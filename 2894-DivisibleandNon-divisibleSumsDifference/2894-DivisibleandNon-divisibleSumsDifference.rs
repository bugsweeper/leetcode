// Last updated: 27.05.2025, 08:12:53
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let n2 = n / m;
        n * (n + 1) / 2 - m * n2 * (n2 + 1)
    }
}