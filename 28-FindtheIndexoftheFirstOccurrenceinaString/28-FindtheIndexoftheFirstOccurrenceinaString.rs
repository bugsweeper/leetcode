impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|index| index as i32).unwrap_or(-1)
    }
}