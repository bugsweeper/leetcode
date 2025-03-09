impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        'substring_length: for substring_length in 1..=s.len() / 2 {
            if s.len() % substring_length != 0 {
                continue;
            }
            for start in (substring_length..s.len()).step_by(substring_length) {
                if s[0..substring_length] != s[start..start + substring_length] {
                    continue 'substring_length
                }
            }
            return true;
        }
        false
    }
}