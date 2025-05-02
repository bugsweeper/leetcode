// Last updated: 02.05.2025, 13:40:44
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let s = s.as_bytes();
        let mut result = Vec::with_capacity(s.len());
        for &letter in s {
            if letter == b'*' {
                let _ = result.pop();
            } else {
                result.push(letter);
            }
        }
        String::from_utf8(result).unwrap()
    }
}