impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() - 1 {
                if nums[j] == nums[j - 1] && j > i + 1 {
                    continue;
                }
                if let Ok(k) = nums[j + 1..].binary_search(&(-nums[i] - nums[j])) {
                    answer.push(vec![nums[i], nums[j], nums[k + j + 1]]);
                }
            }
        }
        answer
    }
}