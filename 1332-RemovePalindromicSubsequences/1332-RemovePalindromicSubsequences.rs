// Last updated: 30.05.2025, 16:36:15
use std::iter::repeat_n;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        let mut result = String::with_capacity(n);
        if n & 1 == 0 {
            result.extend(repeat_n('a', n - 1));
            result.push('b');
        } else {
            result.extend(repeat_n('a', n));
        }
        result
    }
}