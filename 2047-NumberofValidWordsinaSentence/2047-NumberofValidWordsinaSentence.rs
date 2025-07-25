// Last updated: 25.07.2025, 13:37:57
use std::collections::HashSet;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut seen = HashSet::with_capacity(arr.len());
        let mut repeated = HashSet::with_capacity(arr.len());
        for item in &arr {
            if !seen.insert(item) {
                repeated.insert(item);
            }
        }
        arr.iter()
            .filter(|item| !repeated.contains(item))
            .nth(k as usize - 1)
            .cloned()
            .unwrap_or(String::new())
    }
}