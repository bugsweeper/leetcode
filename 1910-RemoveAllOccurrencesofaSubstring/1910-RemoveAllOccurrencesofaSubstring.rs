impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut old_len = s.len();
        let mut result = s.replacen(&part, "", 1);
        while old_len != result.len() {
            old_len = result.len();
            result = result.replacen(&part, "", 1);
        }
        result
    }
}