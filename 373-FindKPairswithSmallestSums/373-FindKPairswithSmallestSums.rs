// Last updated: 08.04.2025, 13:12:56
struct Cell {
    sum: i32,
    i1: usize,
    i2: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum.cmp(&other.sum).reverse()
    }
}

impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.sum.eq(&other.sum)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let capacity = k as usize * 2;
        let mut seen = HashSet::with_capacity(capacity);
        let mut queue = BinaryHeap::with_capacity(capacity);
        queue.push(Cell {
            sum: nums1[0] + nums2[0],
            i1: 0,
            i2: 0,
        });
        seen.insert((0, 0));
        let mut result = Vec::with_capacity(k as usize);
        for _ in 0..k {
            let Cell { i1, i2, .. } = queue.pop().unwrap();
            let (val1, val2) = (nums1[i1], nums2[i2]);
            result.push(vec![val1, val2]);
            let (next_i1, next_i2) = (i1 + 1, i2 + 1);
            if next_i1 < nums1.len() && !seen.contains(&(next_i1, i2)) {
                seen.insert((next_i1, i2));
                queue.push(Cell {
                    sum: nums1[next_i1] + val2,
                    i1: next_i1,
                    i2,
                });
            }
            if next_i2 < nums2.len() && !seen.contains(&(i1, next_i2)) {
                seen.insert((i1, next_i2));
                queue.push(Cell {
                    sum: val1 + nums2[next_i2],
                    i1,
                    i2: next_i2,
                });
            }
        }
        result
    }
}