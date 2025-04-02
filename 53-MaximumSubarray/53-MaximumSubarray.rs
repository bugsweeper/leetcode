// Last updated: 02.04.2025, 10:08:00
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_num = *nums.first().unwrap();
        let mut max_grow = max_num;
        for num in nums {
            max_num = max_num.max(num);
            sum += num;
            if sum < min_sum {
                min_sum = sum;
            } else {
                max_grow = max_grow.max(sum - min_sum);
            }
        }
        if max_num <= 0 {
            max_num
        } else {
            max_grow
        }
    }
}