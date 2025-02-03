use std::cmp::Ordering;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut max_subarray_length = 0;
        let mut current_subarray_length: i32 = 0;
        let mut prev_num = *nums.first().unwrap();
        for num in nums.into_iter().skip(1) {
            match num.cmp(&prev_num) {
                Ordering::Equal => {
                    max_subarray_length = max_subarray_length.max(current_subarray_length.abs());
                    current_subarray_length = 0;
                }
                Ordering::Greater => {
                    current_subarray_length = if current_subarray_length < 0 {
                        max_subarray_length = max_subarray_length.max(-current_subarray_length);
                        1
                    } else {
                        current_subarray_length + 1
                    }
                }
                Ordering::Less => {
                    current_subarray_length = if current_subarray_length > 0 {
                        max_subarray_length = max_subarray_length.max(current_subarray_length);
                        -1
                    } else {
                        current_subarray_length - 1
                    }
                }
            }
            prev_num = num
        }
        max_subarray_length.max(current_subarray_length.abs()) + 1
    }
}