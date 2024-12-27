impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut maximum_reachable_index = 0;
        for (index, num) in nums.into_iter().enumerate() {
            if index > maximum_reachable_index {
                return false;
            }
            maximum_reachable_index = maximum_reachable_index.max(index + num as usize);
        }
        return true;
    }
}