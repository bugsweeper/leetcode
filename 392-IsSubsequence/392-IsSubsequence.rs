impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let mut t = t.as_bytes().into_iter();
        let mut position = 0;
        for letter in s {
            position = if let Some(position) = t.position(|c| c == letter) {
                position
            } else {
                return false;
            }
        }
        return true;
    }
}