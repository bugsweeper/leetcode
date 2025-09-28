// Last updated: 28.09.2025, 15:15:23
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|num| -num);
        for slice in nums.windows(3) {
            let &[a, b, c] = slice else {
                unimplemented!();
            };
            if a < b + c {
                return a + b + c;
            }
        }
        0
    }
}