impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut indexed_nums = nums.iter().copied().enumerate().collect::<Vec<_>>();
        indexed_nums.sort_unstable_by_key(|(_, value)| *value);
        let mut nums = nums;
        for slice in indexed_nums.chunk_by(|&(_, left), &(_, right)| right - left <= limit) {
            let mut indexes = slice.iter().map(|(index, _)| *index).collect::<Vec<_>>();
            indexes.sort_unstable();
            for (index, &(_, value)) in indexes.into_iter().zip(slice.iter()) {
                *unsafe { nums.get_unchecked_mut(index) } = value;
            }
        }
        nums
    }
}