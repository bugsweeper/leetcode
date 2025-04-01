// Last updated: 01.04.2025, 13:24:29
use std::cmp::Ordering;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut sum = 1;
        for i in 2..=num.isqrt() {
            if num % i == 0 {
                sum += i;
                let other = num / i;
                if i != other {
                    sum += other
                }
                if sum > num {
                    return false;
                }
            }
        }
        sum == num
    }
}