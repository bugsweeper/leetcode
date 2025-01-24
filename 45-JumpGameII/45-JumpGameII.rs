impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jump_count = 0;
        let mut reached_position = 0;
        let mut start_position = 0;
        while reached_position < nums.len() - 1 {
            jump_count += 1;
            let mut next_reached_position = reached_position;
            for (index, &jump_length) in nums[..=reached_position].iter().enumerate().skip(start_position) {
                next_reached_position = next_reached_position.max(index + jump_length as usize);
            }
            start_position = reached_position + 1;
            reached_position = next_reached_position;
        }
        jump_count
    }
}