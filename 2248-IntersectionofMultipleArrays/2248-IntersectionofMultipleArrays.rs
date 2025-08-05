// Last updated: 05.08.2025, 12:20:59
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let should_be_seen_times = nums.len();
        let mut seen_times = vec![0; 1001];
        for nums in nums {
            for num in nums {
                seen_times[num as usize] += 1;
            }
        }
        seen_times
            .into_iter()
            .enumerate()
            .filter_map(|(num, seen_times)| {
                if seen_times == should_be_seen_times {
                    Some(num as i32)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}