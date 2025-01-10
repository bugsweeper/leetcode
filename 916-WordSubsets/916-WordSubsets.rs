use std::collections::HashMap;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn to_statistic(word: &str) -> HashMap<u8, i32> {
    let mut result = HashMap::with_capacity(ABC_LEN);
    for &c in word.as_bytes() {
        *result.entry(c).or_default() += 1;
    }
    result
}

#[inline]
fn merge(into: &mut HashMap<u8, i32>, from: HashMap<u8, i32>) {
    for (c, count) in from {
        into.entry(c)
            .and_modify(|value| {
                if *value < count {
                    *value = count
                }
            })
            .or_insert(count);
    }
}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut words2_statistic = HashMap::with_capacity(ABC_LEN);
        for word2 in words2 {
            merge(&mut words2_statistic, to_statistic(&word2));
        }
        words1
            .into_iter()
            .filter(|word1| {
                let word1_statistic = to_statistic(word1);
                for (key, count) in &words2_statistic {
                    if word1_statistic.get(key).unwrap_or(&0) < count {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}