// Last updated: 04.04.2025, 11:04:02
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut size = nums.len();
        let max_index = nums.len() - 1;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if middle != max_index && nums[middle] < nums[middle + 1] {
                base = middle;
            }
            size -= half;
        }
        (if base == max_index || nums[base] > nums[base + 1] {
            base
        } else {
            base + 1
        }) as i32
    }
}