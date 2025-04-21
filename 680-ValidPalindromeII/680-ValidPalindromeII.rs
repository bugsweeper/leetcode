// Last updated: 21.04.2025, 10:51:40
fn valid_palindrome(bytes: &[u8], can_delete: bool) -> bool {
    for (index, (&left, &right)) in bytes.iter().zip(bytes.iter().rev()).take(bytes.len() / 2).enumerate() {
        if left != right {
            return can_delete && (valid_palindrome(&bytes[index..bytes.len() - index - 1], false) || valid_palindrome(&bytes[index + 1..bytes.len() - index], false))
        }
    }
    true
}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        valid_palindrome(s.as_bytes(), true)
    }
}