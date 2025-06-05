// Last updated: 05.06.2025, 12:11:07
use std::cmp::Ordering;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        // at start every character would be replaced to itself
        let mut replacement: [u8; ABC_LEN] = core::array::from_fn(|i| i as u8 + b'a');
        let mut group_count = ABC_LEN;
        for (&c1, &c2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
            let mut min_replacement = replacement[(c1 - b'a') as usize];
            let mut max_replacement = replacement[(c2 - b'a') as usize];
            match min_replacement.cmp(&max_replacement) {
                Ordering::Equal => continue,
                Ordering::Greater => {
                    (min_replacement, max_replacement) = (max_replacement, min_replacement)
                }
                _ => {}
            }
            group_count -= 1;
            for replacement in replacement.iter_mut() {
                if *replacement == max_replacement {
                    *replacement = min_replacement;
                }
            }
            if group_count == 1 {
                break;
            }
        }
        if group_count == ABC_LEN {
            base_str
        } else {
            base_str
                .as_bytes()
                .iter()
                .map(|&byte| (replacement[(byte - b'a') as usize]) as char)
                .collect::<String>()
        }
    }
}