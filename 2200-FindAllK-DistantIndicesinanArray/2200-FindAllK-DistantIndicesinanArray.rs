// Last updated: 24.06.2025, 11:05:21
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let k_distant_indices = Vec::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .filter_map(|(index, num)| if num == key { Some(index) } else { None })
            .fold(
                (k_distant_indices, 0),
                |(mut indices, start_index), index| {
                    let start = index.saturating_sub(k).max(start_index);
                    let end = indices.capacity().min(index + k + 1);
                    indices.extend(start as i32..end as i32);
                    (indices, end)
                },
            )
            .0
    }
}