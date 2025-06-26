// Last updated: 26.06.2025, 11:17:25
impl Solution {
    pub fn is_three(n: i32) -> bool {
        if n < 4 {
            return false;
        }
        let square_root = n.isqrt();
        if square_root * square_root != n {
            return false;
        }
        for prime in [2, 3, 5, 7] {
            if square_root == prime {
                return true;
            }
            if square_root % prime == 0 {
                return false;
            }
        }
        true
    }
}