use std::collections::HashMap;

fn get_counts(nums: Vec<i32>) -> HashMap<i32, usize> {
    let mut result = HashMap::with_capacity(nums.len());
    for num in nums {
        *result.entry(num).or_insert(0) += 1;
    }
    result
}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let max_len = nums1.len() + nums2.len();
        let mut nums1 = get_counts(nums1);
        let mut nums2 = get_counts(nums2);
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let mut result = Vec::with_capacity(max_len);
        for (value, count) in nums1 {
            result
                .extend(std::iter::repeat(value).take(count.min(*nums2.get(&value).unwrap_or(&0))));
        }
        result
    }
}