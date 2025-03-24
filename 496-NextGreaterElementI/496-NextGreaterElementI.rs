// Last updated: 24.03.2025, 11:54:10
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut decreasing_stack = Vec::with_capacity(nums2.len());
        let mut num2greater = std::collections::HashMap::with_capacity(nums2.len());
        let mut prev_num = i32::MAX;
        for num in nums2 {
            if num > prev_num {
                while let Some(lower_num) = decreasing_stack.pop() {
                    if lower_num < num {
                        num2greater.insert(lower_num, num);
                    } else {
                        decreasing_stack.push(lower_num);
                        break;
                    }
                }
            }
            decreasing_stack.push(num);
            prev_num = num;
        }
        nums1.into_iter().map(|num| *num2greater.get(&num).unwrap_or(&-1)).collect()
    }
}