// Last updated: 07.04.2025, 13:55:38
use std::cmp::Ordering;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 1 {
            return false;
        }
        let sum = sum as usize / 2;
        let mut combined = vec![false; sum as usize];
        combined[0] = true;
        let mut max_sum = 0;
        for num in nums {
            let num = num as usize;
            match num.cmp(&sum) {
                Ordering::Equal => return true,
                Ordering::Greater => continue,
                _ => {}
            }
            if combined[sum - num] {
                return true;
            }
            for sub_sum in (0..=max_sum.min(sum - num - 1)).rev() {
                if combined[sub_sum] {
                    combined[sub_sum + num] = true;
                }
            }
            max_sum = max_sum + num;
        }
        false
    }
}