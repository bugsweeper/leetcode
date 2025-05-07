// Last updated: 07.05.2025, 15:00:57
const STAT_LEN: usize = (b'z' - b'A' + 1) as usize;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_stats = [0; STAT_LEN];
        for byte in t.as_bytes() {
            t_stats[(byte - b'A') as usize] += 1;
        }
        let mut window_len = usize::MAX;
        let mut slice = s.as_str();
        let mut left = 0;
        let mut right = 0;
        let mut s_stats = [0; STAT_LEN];
        let s_bytes = s.as_bytes();
        let mut remaining_characters = t_stats.iter().copied().filter(|count| *count > 0).count();
        loop {
            if remaining_characters > 0 {
                let right_stat_index = (s_bytes[right] - b'A') as usize;
                right += 1;
                let (s_stat, t_stat) = (
                    s_stats.get_mut(right_stat_index).unwrap(),
                    t_stats[right_stat_index],
                );
                *s_stat += 1;
                if *s_stat == t_stat {
                    remaining_characters -= 1;
                    if remaining_characters > 0 && right == s.len() {
                        break;
                    }
                } else if right == s.len() {
                    break;
                }
            } else {
                let left_stat_index = (s_bytes[left] - b'A') as usize;
                let (s_stat, t_stat) = (
                    s_stats.get_mut(left_stat_index).unwrap(),
                    t_stats[left_stat_index],
                );
                if *s_stat == t_stat {
                    remaining_characters += 1;
                    if window_len > right - left {
                        slice = &s[left..right];
                        window_len = right - left;
                    }
                    if right == s.len() {
                        break;
                    }
                }
                left += 1;
                *s_stat -= 1;
            }
        }
        if window_len == usize::MAX {
            String::new()
        } else {
            slice.into()
        }
    }
}