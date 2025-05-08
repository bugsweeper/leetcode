// Last updated: 08.05.2025, 13:08:18
use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut alice_box_set = HashSet::with_capacity(alice_sizes.len());
        let mut alice_candy_count = 0;
        for size in alice_sizes {
            alice_candy_count += size;
            alice_box_set.insert(size);
        }
        let mut bob_box_set = HashSet::with_capacity(bob_sizes.len());
        let mut bob_candy_count = 0;
        for size in bob_sizes {
            bob_candy_count += size;
            bob_box_set.insert(size);
        }
        let diff = (bob_candy_count - alice_candy_count) / 2;
        for alice_box_size in alice_box_set {
            let bob_box_size = alice_box_size + diff;
            if bob_box_set.contains(&bob_box_size) {
                return vec![alice_box_size, bob_box_size];
            }
        }
        vec![]
    }
}