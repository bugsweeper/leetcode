// Last updated: 06.05.2025, 15:46:52
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        while n & 1 == 0 {
            n >>= 1;
        }
        let mut max_gap = 0;
        let mut current_gap = 1;
        n >>= 1;
        while n > 0 {
            if n & 1 == 1 {
                max_gap = max_gap.max(current_gap);
                current_gap = 1;
            } else {
                current_gap += 1;
            }
            n >>= 1;
        }
        max_gap
    }
}