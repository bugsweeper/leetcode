use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        answer.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        while j < nums.len() && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                    }
                    Ordering::Greater => k -= 1,
                    Ordering::Less => j += 1,
                }
            }
        }
        answer
    }
}
