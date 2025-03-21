// Last updated: 21.03.2025, 16:32:04
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as u64;
        let mut base = 1;
        let mut size = n;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if middle * (middle + 1) / 2 <= n {
                base = middle;
            }
            size -= half;
        }
        base as i32
    }
}