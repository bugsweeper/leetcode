// Last updated: 13.05.2025, 14:22:56
use std::collections::VecDeque;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const MODULO: i32 = 1_000_000_007;
const MODULO64: i64 = MODULO as i64;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut a_transformations = VecDeque::from(vec![0; ABC_LEN]);
        a_transformations[0] = 1;
        for _ in 0..t {
            let z = a_transformations.pop_back().unwrap();
            a_transformations.push_front(z);
            let b = a_transformations.get_mut(1).unwrap();
            *b = (*b + z) % MODULO;
        }
        let mut s_frequency = [0; ABC_LEN];
        for &c in s.as_bytes() {
            s_frequency[(c - b'a') as usize] += 1;
        }
        let mut length = 0;
        for (s_char, frequency) in s_frequency.into_iter().enumerate() {
            for (a_became_char, &count) in a_transformations.iter().enumerate() {
                let mut count = frequency as i64 * count as i64;
                if s_char + a_became_char >= ABC_LEN {
                    count <<= 1;
                }
                length = (length + (count % MODULO64) as i32) % MODULO;
            }
        }
        length
    }
}