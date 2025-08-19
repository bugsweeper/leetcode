// Last updated: 19.08.2025, 09:38:12
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut zero_count = 0;
        let mut subarray_count = 0;
        for num in nums {
            if num == 0 {
                zero_count += 1;
            } else if zero_count != 0 {
                subarray_count += (zero_count * (zero_count + 1)) >> 1;
                zero_count = 0;
            }
        }
        if zero_count == 0 {
            subarray_count
        } else {
            subarray_count + ((zero_count * (zero_count + 1)) >> 1)
        }
    }
}