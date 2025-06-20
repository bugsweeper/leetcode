// Last updated: 20.06.2025, 14:55:08
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut size = nums.len();
        let first = nums[0];
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if nums[middle] > first {
                base = middle;
            }
            size -= half;
        }
        if base == nums.len() - 1 {
            first
        } else if nums[base] > first {
            nums[base + 1]
        } else {
            for num in nums.into_iter().skip(base) {
                if num < first {
                    return num;
                }
            }
            first
        }
    }
}