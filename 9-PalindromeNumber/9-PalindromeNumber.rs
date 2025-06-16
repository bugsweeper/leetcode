// Last updated: 16.06.2025, 15:36:35
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        let power_of_10 = x.ilog10();
        // Put to stack right half of number
        let mut stack = (0..power_of_10.div_ceil(2))
            .map(|_| {
                let digit = x % 10;
                x /= 10;
                digit
            })
            .collect::<Vec<_>>();
        // Exclude middle digit for odd length number
        if power_of_10 % 2 == 0 {
            x /= 10;
        }
        // Compare digits of left half with digits from stack
        for _ in 0..power_of_10.div_ceil(2) {
            if x % 10 != stack.pop().unwrap() {
                return false;
            }
            x /= 10;
        }
        true
    }
}