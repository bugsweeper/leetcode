// Last updated: 24.04.2025, 13:13:05
#[inline]
fn is_self_dividing(number: i32) -> bool {
    let mut remaining = number;
    while remaining > 0 {
        let digit = remaining % 10;
        if digit == 0 || number % digit != 0 {
            return false;
        }
        remaining /= 10;
    }
    true
}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut array = Vec::with_capacity(339);
        for i in left..=right {
            if is_self_dividing(i) {
                array.push(i);
            }
        }
        array
    }
}