// Last updated: 15.08.2025, 08:47:18
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let mut n = n;
        while n & 3 == 0 {
            n >>= 2;
        }
        n == 1
    }
}