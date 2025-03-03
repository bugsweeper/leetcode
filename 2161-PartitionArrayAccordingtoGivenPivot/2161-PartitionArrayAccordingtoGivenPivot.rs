use std::cmp::Ordering;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut less = Vec::with_capacity(nums.len());
        let mut greater = Vec::with_capacity(nums.len());
        for &num in &nums {
            match num.cmp(&pivot) {
                Ordering::Less => less.push(num),
                Ordering::Equal => {}
                Ordering::Greater => greater.push(num),
            }
        }
        nums[..less.len()].copy_from_slice(&less[..]);
        let greater_start = nums.len() - greater.len();
        for num in &mut nums[less.len()..greater_start] {
            *num = pivot
        }
        nums[greater_start..].copy_from_slice(&greater[..]);
        nums
    }
}
