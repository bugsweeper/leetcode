// Last updated: 19.06.2025, 14:05:07
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_index = match rule_key.as_bytes()[0] {
            b't' => 0,
            b'c' => 1,
            b'n' => 2,
            _ => unimplemented!(),
        };
        items
            .into_iter()
            .filter(|item| item[rule_index] == rule_value)
            .count() as i32
    }
}