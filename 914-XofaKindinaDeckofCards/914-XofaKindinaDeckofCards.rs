// Last updated: 12.05.2025, 12:14:58
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut cards_count = HashMap::with_capacity(deck.len());
        for card in deck {
            cards_count.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }
        let counts = cards_count.values().copied().collect::<Vec<_>>();
        let mut iter = counts.into_iter();
        let mut group_size = iter.next().unwrap();
        for mut count in iter {
            while count % group_size != 0 {
                (count, group_size) = (group_size, count % group_size)
            }
            if group_size == 1 {
                return false;
            }
        }
        group_size > 1
    }
}