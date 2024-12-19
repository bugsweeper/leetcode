use std::cmp::Reverse;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        if s.is_empty() {
            return s;
        }
        let mut s = s;
        let s_bytes = unsafe { s.as_bytes_mut() };
        s_bytes.sort_unstable_by_key(|&symbol| Reverse(symbol));
        let repeat_limit = repeat_limit as usize;
        let mut max_index = 0;
        let mut max = s_bytes[0];
        let mut maybe_next_index = s_bytes.iter().position(|&symbol| symbol != max);
        'top: while let Some(mut next_index) = maybe_next_index {
            while max_index + repeat_limit < next_index {
                s_bytes.swap(max_index + repeat_limit, next_index);
                max_index += repeat_limit + 1;
                next_index += 1;
                if next_index >= s_bytes.len() {
                    break 'top;
                }
            }
            max_index = next_index;
            max = s_bytes[max_index];
            maybe_next_index = s_bytes
                .iter()
                .skip(max_index + 1)
                .position(|&symbol| symbol != max)
                .and_then(|index| {
                    let new_index = index + max_index + 1;
                    if new_index < s_bytes.len() {
                        Some(new_index)
                    } else {
                        None
                    }
                });
        }
        if max_index + repeat_limit < s.len() {
            s = s[..max_index + repeat_limit].into();
        }
        s
    }
}