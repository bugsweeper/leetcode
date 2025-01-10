const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn to_statistic(word: &str) -> Vec<i32> {
    let mut result = Vec::with_capacity(ABC_LEN);
    for &c in word.as_bytes() {
        let c = c as usize;
        if c >= result.len() {
            result.resize(c + 1, 0);
        }
        *unsafe { result.get_unchecked_mut(c) } += 1;
    }
    result
}

#[inline]
fn merge(into: &mut Vec<i32>, from: Vec<i32>) {
    if into.len() < from.len() {
        into.resize(from.len(), 0);
    }
    for (into, from) in into.iter_mut().zip(from.into_iter()) {
        *into = (*into).max(from);
    }
}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut words2_statistic = Vec::with_capacity(ABC_LEN);
        for word2 in words2 {
            merge(&mut words2_statistic, to_statistic(&word2));
        }
        words1
            .into_iter()
            .filter(|word1| {
                let word1_statistic = to_statistic(word1);
                if word1_statistic.len() < words2_statistic.len() {
                    return false;
                }
                for (count1, &count2) in word1_statistic.into_iter().zip(words2_statistic.iter()) {
                    if count1 < count2 {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}