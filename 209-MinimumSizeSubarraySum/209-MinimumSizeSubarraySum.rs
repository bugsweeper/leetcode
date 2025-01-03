impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut sum = unsafe { *nums.get_unchecked(right) };
        let mut min_size = usize::MAX;
        loop {
            if sum < target {
                right += 1;
                if right >= nums.len() {
                    break;
                }
                sum += unsafe { nums.get_unchecked(right) };
            } else {
                min_size = min_size.min(right - left + 1);
                sum -= unsafe { nums.get_unchecked(left) };
                left += 1;
            }
        }
        if min_size == usize::MAX { 0 } else { min_size as i32 }
    }
}