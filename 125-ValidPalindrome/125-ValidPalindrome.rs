impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s
            .as_bytes()
            .iter()
            .filter(|ascii| ascii.is_ascii_alphanumeric())
            .map(|ascii| ascii.to_ascii_lowercase());
        loop {
            match (iter.next(), iter.next_back()) {
                (Some(left), Some(right)) if left != right => return false,
                (None, None) => break,
                _ => {}
            }
        }
        true
    }
}