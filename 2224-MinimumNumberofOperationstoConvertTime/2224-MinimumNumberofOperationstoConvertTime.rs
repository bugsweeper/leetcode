// Last updated: 04.08.2025, 16:55:50
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        if nums.len() == 1 {
            let mut nums = nums;
            let mut nums = nums.pop().unwrap();
            nums.sort_unstable();
            return nums;
        }
        let iterations = nums.len() - 2;
        let mut iter = nums.into_iter();
        let mut set = iter.next().unwrap().into_iter().collect::<HashSet<i32>>();
        for nums in iter {
            let other_set = nums.into_iter().collect::<HashSet<_>>();
            set = set.intersection(&other_set).copied().collect::<HashSet<_>>();
        }
        let mut nums = set.into_iter().collect::<Vec<_>>();
        nums.sort_unstable();
        nums
    }
}