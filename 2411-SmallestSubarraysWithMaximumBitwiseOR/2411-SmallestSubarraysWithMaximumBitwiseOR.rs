// Last updated: 29.07.2025, 10:29:12
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut answer = vec![1; nums.len()];
        // Going forward and propagating sum backward (yes it destroys data, but you don't need it anymore)
        for i in 1..nums.len() {
            let mut sum = nums[i];
            // while sum contains unique bit for previous items add this bit and update answer
            for j in (0..i).rev() {
                let num = &mut nums[j];
                sum |= *num;
                if sum == *num {
                    // all previous items contain all bits of sum
                    break;
                } else {
                    answer[j] = (i - j + 1) as i32;
                    *num = sum;
                }
            }
        }
        answer
    }
}