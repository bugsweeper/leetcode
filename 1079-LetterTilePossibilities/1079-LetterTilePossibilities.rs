use std::collections::HashMap;

const ABC_LEN: usize = (b'Z' - b'A' + 1) as usize;

#[inline]
fn get_statistic(s: &str) -> Vec<i32> {
    let mut statistic = [0; ABC_LEN];
    for &c in s.as_bytes() {
        *unsafe { statistic.get_unchecked_mut((c - b'A') as usize) } += 1;
    }
    statistic.into_iter().filter(|&count| count > 0).collect()
}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let available = get_statistic(&tiles);
        let mut num_per_used = HashMap::new();
        num_per_used.insert(vec![0; available.len()], 1);
        let mut result = 0;
        for _ in 0..tiles.len() {
            let mut next_num_per_used = HashMap::with_capacity(available.len() * num_per_used.len());
            for (combination, count) in &num_per_used {
                for letter_index in 0..available.len() {
                    if combination[letter_index] < available[letter_index] {
                        let mut new_combination = combination.clone();
                        new_combination[letter_index] += 1;
                        *next_num_per_used.entry(new_combination).or_insert(0) += count;
                        result += count;
                    }
                }
            }
            std::mem::swap(&mut num_per_used, &mut next_num_per_used);
        }
        result
    }
}