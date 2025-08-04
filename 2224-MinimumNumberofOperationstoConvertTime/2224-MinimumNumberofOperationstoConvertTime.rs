// Last updated: 04.08.2025, 13:41:24
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let (mut digits, mut odds, mut evens, mut num) = (
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            num,
        );
        // Decompose to digits
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            digits.push(digit);
            if digit & 1 == 1 {
                odds.push(digit);
            } else {
                evens.push(digit);
            }
        }
        // Sort separately digits
        odds.sort_unstable();
        evens.sort_unstable();
        // Combine sorted digits by parity
        for digit in digits.iter_mut().rev() {
            *digit = if *digit & 1 == 1 {
                odds.pop()
            } else {
                evens.pop()
            }
            .unwrap();
        }
        // Recompose
        for digit in digits.into_iter().rev() {
            num = num * 10 + digit;
        }
        num
    }
}