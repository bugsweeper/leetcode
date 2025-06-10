// Last updated: 10.06.2025, 10:51:29
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [0; b'z' as usize + 1];
        for &byte in s.as_bytes() {
            freq[byte as usize] += 1;
        }
        let mut max_odd_freq = 0;
        let mut min_even_freq = i32::MAX;
        for &freq in &freq[b'a' as usize..=b'z' as usize] {
            if freq == 0 {
                continue;
            }
            if freq & 1 == 0 {
                min_even_freq = min_even_freq.min(freq);
            } else {
                max_odd_freq = max_odd_freq.max(freq);
            }
        }
        max_odd_freq - min_even_freq
    }
}