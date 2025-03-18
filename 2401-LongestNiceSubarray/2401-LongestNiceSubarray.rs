impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;
        let mut sum = nums[0];
        let mut result = 1;
        while right < nums.len() {
            let next_value = nums[right];
            if sum & next_value == 0 {
                sum |= next_value;
            } else {
                result = result.max(right - left);
                if result == 30 {
                    break;
                }
                while sum & next_value != 0 {
                    sum &= !nums[left];
                    left += 1;
                }
                sum |= next_value;
            }
            right += 1;
        }
        result.max(right - left) as i32
    }
}