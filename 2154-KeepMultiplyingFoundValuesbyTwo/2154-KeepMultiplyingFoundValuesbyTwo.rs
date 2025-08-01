// Last updated: 01.08.2025, 12:10:16
use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let mut nums2 = nums2.into_iter().collect::<HashSet<_>>();
        vec![nums1.difference(&nums2).copied().collect::<Vec<_>>(), nums2.difference(&nums1).copied().collect::<Vec<_>>()]
    }
}