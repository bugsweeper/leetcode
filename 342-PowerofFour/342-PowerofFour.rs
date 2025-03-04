impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let mut n = n;
        while n > 3 {
            if n & 3 != 0 {
                return false;
            }
            n >>= 2;
        }
        n == 1
    }
}