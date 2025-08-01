// Last updated: 01.08.2025, 11:15:43
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);
        let mut num = num;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable();
        (digits[0] + digits[1]) * 10 + digits[2] + digits[3]
    }
}