impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        if nums2.len() % 2 == 1 {
            result = nums1.iter().fold(result, |acc, &item| acc ^ item);
        }
        if nums1.len() % 2 == 1 {
            result = nums2.iter().fold(result, |acc, &item| acc ^ item);
        }
        result
    }
}