// Last updated: 30.05.2025, 16:29:13
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut count = [0; ABC_LEN];
        for &byte in s.as_bytes() {
            count[(byte - b'a') as usize] += 1;
        }
        let mut result = String::with_capacity(s.len());
        while result.len() < s.len() {
            for (index, count) in count.iter_mut().enumerate() {
                if *count > 0 {
                    result.push((index as u8 + b'a') as char);
                    *count -= 1;
                }
            }
            for (index, count) in count.iter_mut().enumerate().rev() {
                if *count > 0 {
                    result.push((index as u8 + b'a') as char);
                    *count -= 1;
                }
            }
        }
        result
    }
}