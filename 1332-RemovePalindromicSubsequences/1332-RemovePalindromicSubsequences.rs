// Last updated: 30.05.2025, 13:49:28
fn is_palindrome(bytes: &[u8]) -> bool {
    for (&left, &right) in bytes.iter().zip(bytes.iter().rev()).take(bytes.len() / 2) {
        if left != right {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if is_palindrome(s.as_bytes()) {
            1
        } else {
            2
        }
    }
}