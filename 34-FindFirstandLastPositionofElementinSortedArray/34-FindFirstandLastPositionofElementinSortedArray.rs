use std::cmp::Ordering;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        // The problem could be solved with just two `partition_point` calls
        // but second call would search in full range in this case,
        // while this boilerplate code uses some info from previous search
        // which could reduce amount of jumps at second search
        let mut base = 0;
        let mut size = nums.len();
        let mut ceil = size;
        let mut base_for_last = base;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            match target.cmp(unsafe { nums.get_unchecked(middle) }) {
                Ordering::Equal => {
                    base_for_last = middle;
                }
                Ordering::Greater => {
                    base = middle;
                }
                Ordering::Less => {
                    ceil = middle;
                }
            }
            size -= half;
        }
        if nums[base] == target {
            // Looks like first element equals target, just do nothing
        } else if base + 1 >= nums.len() || nums[base + 1] != target {
            return vec![-1, -1];
        } else {
            base += 1;
        }
        if nums[base_for_last] != target {
            base_for_last = base;
        }
        size = ceil - base_for_last;
        while size > 1 {
            let half = size / 2;
            let middle = base_for_last + half;
            if target == *unsafe { nums.get_unchecked(middle) } {
                base_for_last = middle;
            }
            size -= half;
        }
        vec![base as i32, base_for_last as i32]
    }
}