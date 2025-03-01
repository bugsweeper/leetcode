impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut index = 1;
        let mut nums = nums;
        while index < nums.len() {
            if nums[index - 1] == nums[index] {
                nums[index - 1] <<= 1;
                nums[index] = 0;
                index += 2;
            } else {
                index += 1;
            }
        }
        let mut left = 0;
        while left < nums.len() && nums[left] > 0 {
            left += 1;
        }
        if left < nums.len() {
            let mut right = left + 1;
            while right < nums.len() {
                if nums[right] != 0 {
                    nums.swap(left, right);
                    left += 1;
                }
                right += 1;
            }
        }
        nums
    }
}