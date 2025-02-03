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
        let mut stack = (0..(power_of_10 + 1) / 2).map(|_| {
            let digit = x % 10;
            x /= 10;
            digit
        }).collect::<Vec<_>>();
        if power_of_10 % 2 == 0 {
            x /= 10;
        }
        for _ in 0..(power_of_10 + 1) / 2 {
            if x % 10 != stack.pop().unwrap() {
                return false;
            }
            x /= 10;
        }
        true
    }
}