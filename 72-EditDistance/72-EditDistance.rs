// Last updated: 29.04.2025, 11:57:25
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.is_empty() {
            return word2.len() as i32;
        }
        if word2.is_empty() {
            return word1.len() as i32;
        }
        if word1 == word2 {
            return 0;
        }
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        // Levenshtein distance
        let mut prev = (0..=word1.len()).collect::<Vec<_>>();
        for (i, word2_letter) in word2.iter().enumerate() {
            let (mut replace, mut insert) = (prev[0], i + 1);
            let mut cur = vec![0; word1.len() + 1];
            cur[0] = insert;
            for ((delete, decision), word1_letter) in prev
                .into_iter()
                .zip(cur.iter_mut())
                .skip(1)
                .zip(word1.iter())
            {
                let replace_cost = if word2_letter == word1_letter { 0 } else { 1 };
                *decision = (delete.min(insert) + 1).min(replace + replace_cost);
                (replace, insert) = (delete, *decision)
            }
            prev = cur;
        }
        prev.last().copied().unwrap() as i32
    }
}