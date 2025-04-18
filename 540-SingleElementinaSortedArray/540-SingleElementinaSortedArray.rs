// Last updated: 18.04.2025, 11:15:47
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut size = nums.len();
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if (middle ^ 1) < nums.len() && nums[middle] == nums[middle ^ 1] {
                base = middle;
            }
            size -= half;
        }
        if base < nums.len() - 1 && nums[base] == nums[base ^ 1] {
            nums[base + (2 - (base & 1))]
        } else {
            nums[base]
        }
    }
}
