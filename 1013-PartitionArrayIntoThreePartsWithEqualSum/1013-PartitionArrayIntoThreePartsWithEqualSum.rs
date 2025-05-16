// Last updated: 16.05.2025, 22:02:38
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut start_index = 1;
        while start_index < s.len() {
            let mut brackets = 1;
            for (index, &byte) in s.as_bytes().iter().enumerate().skip(start_index) {
                if byte == b')' {
                    brackets -= 1;
                    if brackets == 0 {
                        if start_index != index - 1 {
                            result.push_str(&s[start_index..index]);
                        }
                        start_index = index + 2;
                        break;
                    }
                } else {
                    brackets += 1;
                }
            }
        }
        result
    }
}