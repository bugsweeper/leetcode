impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut prev_num = *nums.first().unwrap();
        let mut max_sum = prev_num;
        let mut sum = prev_num;
        for num in nums.into_iter().skip(1) {
            if num > prev_num {
                sum += num;
            } else {
                max_sum = max_sum.max(sum);
                sum = num;
            }
            prev_num = num;
        }
        max_sum.max(sum)
    }
}