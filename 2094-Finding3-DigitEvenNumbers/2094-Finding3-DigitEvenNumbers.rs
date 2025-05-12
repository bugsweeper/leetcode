// Last updated: 12.05.2025, 09:44:43
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut acceptable_count = [0; 10];
        for digit in digits {
            acceptable_count[digit as usize] += 1;
        }
        let mut result = Vec::with_capacity(450);
        let mut number = 100;
        while number < 999 {
            let mut count = [0; 10];
            let first_digit = (number / 100) as usize;
            if acceptable_count[first_digit] == 0 {
                number += 100;
                continue;
            }
            count[first_digit] += 1;
            let second_digit = (number / 10 % 10) as usize;
            if acceptable_count[second_digit] <= count[second_digit] {
                number += 10;
                continue;
            }
            count[second_digit] += 1;
            let third_digit = (number % 10) as usize;
            if acceptable_count[third_digit] > count[third_digit] {
                result.push(number);
            }
            number += 2;
        }
        result
    }
}