// Last updated: 12.05.2025, 12:35:31
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut bytes = s.as_bytes().to_vec();
        let mut left = 0;
        let mut right = bytes.len() - 1;
        while left < right {
            if let Some(shift) = bytes[left..right]
                .iter()
                .position(|byte| byte.is_ascii_alphabetic())
            {
                left += shift;
            } else {
                break;
            }
            if let Some(shift) = bytes[left + 1..=right]
                .iter()
                .rev()
                .position(|byte| byte.is_ascii_alphabetic())
            {
                right -= shift;
            } else {
                break;
            }
            bytes.swap(left, right);
            left += 1;
            right -= 1;
        }
        String::from_utf8(bytes).unwrap()
    }
}