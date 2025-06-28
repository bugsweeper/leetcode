// Last updated: 28.06.2025, 14:58:33
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == nums.len() {
            return nums;
        }
        let mut indexed_nums = nums.iter().copied().enumerate().collect::<Vec<_>>();
        indexed_nums.sort_unstable_by_key(|(index, num)| *num);
        let summed = if k << 1 < nums.len() {
            let mut summed = vec![false; nums.len()];
            for (index, _) in indexed_nums.into_iter().rev().take(k) {
                summed[index] = true;
            }
            summed
        } else {
            let mut summed = vec![true; nums.len()];
            for (index, _) in indexed_nums.into_iter().take(nums.len() - k) {
                summed[index] = false;
            }
            summed
        };
        let mut answer = Vec::with_capacity(k);
        answer.extend(
            nums
                .into_iter()
                .zip(summed)
                .filter_map(|(num, is_in_k_largest)| if is_in_k_largest {Some(num)} else {None})
        );
        answer
    }
}