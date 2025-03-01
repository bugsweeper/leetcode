impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut n = n;
        for factor in [2, 3, 5] {
            while n % factor == 0 {
                n /= factor;
            }
        }
        n == 1
    }
}