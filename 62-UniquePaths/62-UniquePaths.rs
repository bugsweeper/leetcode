// Last updated: 25.04.2025, 16:16:22
#[inline]
fn c(k: i64, n: i64) -> i64 {
    let mut product = 1;
    for i in 0..k.min(n-k) {
        product = product * (n - i) / (i + 1);
    }
    product
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as i64, n as i64);
        c(m - 1, m + n - 2) as i32
    }
}