// Last updated: 02.07.2025, 16:08:17
use std::cmp::Ordering;
use std::iter::{once, repeat_n};

const MODULO: i64 = 1_000_000_007;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        match word.len().cmp(&k) {
            Ordering::Equal => return 1,
            Ordering::Less => return 0,
            _ => {}
        }
        let (mut prev_byte, mut count) = (word.as_bytes()[0], 0);
        let sequences_len = word
            .bytes()
            .chain(once(b' '))
            .filter_map(|byte| {
                if prev_byte == byte {
                    count += 1;
                    None
                } else {
                    let item = Some(count);
                    prev_byte = byte;
                    count = 1;
                    item
                }
            })
            .collect::<Vec<_>>();
        let unrestricted_combinations = sequences_len
            .iter()
            .copied()
            .fold(1, |combinations, sequence_len| {
                (combinations * sequence_len as i64) % MODULO
            });
        if k <= sequences_len.len() {
            return unrestricted_combinations as i32;
        }
        let lower_than_k = k - sequences_len.len();
        let equal_or_greater_than_k = word.len() - k + 1;
        let min_combinations_len = equal_or_greater_than_k.min(lower_than_k);
        let (mut prev_combinations, mut combinations) =
            (vec![0; min_combinations_len], vec![0; min_combinations_len]);
        // each cell in `combinations` is amount of strings with corresponding amount of characters removed / added (depending of what we are calculating)
        // for example `combinations[0] == 1` because there is single string with no characters removed / added
        prev_combinations[0] = 1_i64;
        let mut parsed_len = 0;
        for (sequence_index, sequence_len) in sequences_len.into_iter().enumerate() {
            parsed_len += sequence_len;
            if sequence_len == 1 {
                continue;
            }
            let (max_combinations, mut sum, mut exclude_sum) = (parsed_len - sequence_index, 0, 0);
            for (combinations, (&prev_combinations, prev_exclude)) in combinations.iter_mut().zip(
                prev_combinations
                    .iter()
                    .zip(repeat_n(0, sequence_len).chain(prev_combinations.iter().copied()))
                    .take(max_combinations),
            ) {
                sum += prev_combinations;
                exclude_sum += prev_exclude;
                *combinations = (sum - exclude_sum) % MODULO;
            }
            std::mem::swap(&mut prev_combinations, &mut combinations);
        }
        (if equal_or_greater_than_k >= lower_than_k {
            unrestricted_combinations + MODULO - prev_combinations.into_iter().sum::<i64>() % MODULO
        } else {
            prev_combinations.into_iter().sum::<i64>()
        } % MODULO) as i32
    }
}