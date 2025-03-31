// Last updated: 31.03.2025, 16:03:39
use std::cmp::Reverse;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score_sorted = score.into_iter().enumerate().collect::<Vec<_>>();
        score_sorted.sort_unstable_by_key(|(_, value)| Reverse(*value));
        let mut score_ranked = score_sorted
            .into_iter()
            .enumerate()
            .map(|(rank_index, (prev_index, _))| {
                (
                    prev_index,
                    match rank_index {
                        0 => "Gold Medal".into(),
                        1 => "Silver Medal".into(),
                        2 => "Bronze Medal".into(),
                        other => (other + 1).to_string(),
                    },
                )
            })
            .collect::<Vec<_>>();
        score_ranked.sort_unstable_by_key(|(index, _)| *index);
        score_ranked.into_iter().map(|(_, rank)| rank).collect()
    }
}