// Last updated: 20.06.2025, 14:37:16
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut base = 0;
        let mut size = x.saturating_add(1);
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if mid <= x / mid {
                base = mid;
            }
            size -= half;
        }
        base
    }
}