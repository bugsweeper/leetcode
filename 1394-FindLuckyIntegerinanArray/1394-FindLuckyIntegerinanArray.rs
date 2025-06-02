// Last updated: 02.06.2025, 15:32:35
impl Solution {
    pub fn reformat(s: String) -> String {
        let mut s = s;
        let mut digits = Vec::new();
        let mut letters = Vec::new();
        for &c in s.as_bytes() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else if c.is_ascii_alphabetic() {
                letters.push(c);
            }
        }
        if digits.len().abs_diff(letters.len()) > 1 {
            return String::new();
        }
        let mut result = String::with_capacity(s.len());
        let mut digit_iter = digits.iter();
        let mut letter_iter = letters.iter();
        if letters.len() > digits.len() {
            result.push(*letter_iter.next().unwrap() as char);
        }
        loop {
            if let Some(digit) = digit_iter.next() {
                result.push(*digit as char);
            }
            if let Some(letter) = letter_iter.next() {
                result.push(*letter as char);
            } else {
                break;
            }
        }
        result
    }
}