// Last updated: 12.06.2025, 09:13:05
impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let first = *nums.first().unwrap();
        let mut prev_num = first;
        let mut distance = 0;
        for num in nums.into_iter().skip(1) {
            distance = distance.max((num - prev_num).abs());
            prev_num = num;
        }
        distance.max((prev_num - first).abs())
    }
}