// Last updated: 12.06.2025, 15:51:28
use std::cmp::Ordering;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut prev_time = 0;
        let mut max_delta = 0;
        let mut slowest_key = b'a';
        for (time, key) in release_times.into_iter().zip(keys_pressed.bytes()) {
            let delta = time - prev_time;
            prev_time = time;
            match delta.cmp(&max_delta) {
                Ordering::Greater => {
                    max_delta = delta;
                    slowest_key = key;
                }
                Ordering::Equal => slowest_key = slowest_key.max(key),
                _ => {}
            }
        }
        slowest_key as char
    }
}