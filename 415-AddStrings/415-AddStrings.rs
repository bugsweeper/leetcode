use std::iter::repeat;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let max_len = num1.len().max(num2.len());
        let (mut buffer, carry) = num1
            .as_bytes()
            .iter()
            .rev()
            .map(|digit| digit - b'0')
            .chain(repeat(0).take(max_len - num1.len()))
            .zip(
                num2.as_bytes()
                    .iter()
                    .rev()
                    .map(|digit| digit - b'0')
                    .chain(repeat(0).take(max_len - num2.len())),
            )
            .fold(
                (Vec::with_capacity(max_len + 1), 0),
                |(mut buffer, carry), (digit1, digit2)| {
                    let result = digit1 + digit2 + carry;
                    buffer.push(result % 10 + b'0');
                    (buffer, result / 10)
                },
            );
        if carry > 0 {
            buffer.push(carry + b'0');
        }
        let end = buffer.len() - 1;
        for i in 0..=end / 2 {
            buffer.swap(i, end - i);
        }
        unsafe { String::from_utf8_unchecked(buffer) }
    }
}