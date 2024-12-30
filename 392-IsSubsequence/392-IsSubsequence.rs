impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let mut t = t.as_bytes().iter();
        for letter in s {
            if !t.any(|c| c == letter) {
                return false;
            }
        }
        true
    }
}
