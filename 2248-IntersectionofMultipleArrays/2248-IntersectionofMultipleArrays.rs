// Last updated: 05.08.2025, 14:59:34
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        // Lets try find first occurrence of digit with greater digit following
        let digit_byte = digit as u8;
        let index = if digit != '9'
            && let Some(index) = number
                .as_bytes()
                .windows(2)
                .position(|slice| slice[0] == digit_byte && slice[1] > digit_byte)
        {
            index
        } else {
            // new number will be lower than current, remove closer from end
            let place_from_end = number
                .bytes()
                .rev()
                .position(|byte| byte == digit_byte)
                .unwrap();
            number.len() - place_from_end - 1
        };
        let mut answer = String::with_capacity(number.len() - 1);
        answer.push_str(&number[..index]);
        answer.push_str(&number[index + 1..]);
        answer
    }
}