// Last updated: 19.05.2025, 11:36:37
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = stones.into_iter().collect::<BinaryHeap<_>>();
        loop {
            if let Some(y) = heap.pop() {
                if let Some(x) = heap.pop() {
                    if y > x {
                        heap.push(y - x);
                    }
                } else {
                    return y;
                }
            } else {
                return 0;
            }
        }
        unimplemented!();
    }
}