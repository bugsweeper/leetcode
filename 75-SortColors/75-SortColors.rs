// Last updated: 17.05.2025, 09:56:49
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count = [0; 3];
        for num in nums.iter_mut() {
            count[*num as usize] += 1;
        }
        let (mut start, mut end) = (0, 0);
        for (index, count) in count.into_iter().enumerate() {
            (start, end) = (end, end + count);
            nums[start..end].fill(index as i32);    
        }
    }
}