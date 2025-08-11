// Last updated: 11.08.2025, 12:50:04
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let goal_byte = letter as u8;
        (s.bytes().filter(|byte| goal_byte.eq(byte)).count() * 100 / s.len()) as i32
    }
}