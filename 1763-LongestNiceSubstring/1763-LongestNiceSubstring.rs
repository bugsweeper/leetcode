// Last updated: 19.06.2025, 14:47:33
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = [false; 10];
        for byte in s.bytes() {
            if byte.is_ascii_digit() {
                let digit = (byte - b'0') as usize;
                match digit {
                    8 => if digits[9] { return 8 },
                    9 => if digits[8] { return 8 },
                    _ => {}
                }
                digits[digit] = true;
            }
        }
        digits
            .into_iter()
            .enumerate()
            .rev()
            .filter_map(|(index, seen)| if seen { Some(index as i32) } else { None })
            .skip(1)
            .next()
            .unwrap_or(-1)
    }
}