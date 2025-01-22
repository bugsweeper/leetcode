impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut previous = *unsafe { nums.get_unchecked(0) };
        let mut cur_len = 1;
        let mut max_len = 1;
        for num in nums {
            match num - previous {
                0 => continue,
                1 => cur_len += 1,
                _ => {
                    max_len = max_len.max(cur_len);
                    cur_len = 1;
                }
            }
            previous = num;
        }
        max_len.max(cur_len)
    }
}