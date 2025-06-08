// Last updated: 08.06.2025, 12:35:21
fn push_starting_from(number: i32, last: i32, result: &mut Vec<i32>) {
    result.push(number);
    let next_number = number * 10;
    if next_number <= last {
        for i in next_number..=last.min(next_number + 9) {
            push_starting_from(i, last, result);
        }
    }
}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        for first_digit in 1..=n.min(9) {
            push_starting_from(first_digit, n, &mut result);
        }
        result
    }
}