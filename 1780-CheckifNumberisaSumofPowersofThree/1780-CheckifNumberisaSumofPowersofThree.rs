impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            let reminder = n % 3;
            if reminder == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}