// Last updated: 22.07.2025, 11:17:27
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut sum, mut max_sum) = (0, 0);
        let mut seen = [false; 10_001];
        let mut left_index = 0;
        for (right_index, &right_num) in nums.iter().enumerate() {
            let right_seen = &mut seen[right_num as usize];
            if *right_seen {
                max_sum = max_sum.max(sum);
                for (new_left_index, &left_num) in nums.iter().enumerate().skip(left_index) {
                    if left_num == right_num {
                        left_index = new_left_index + 1;
                        break;
                    }
                    sum -= left_num;
                    seen[left_num as usize] = false;
                }
            } else {
                sum += right_num;
                *right_seen = true;
            }
        }
        max_sum.max(sum)
    }
}