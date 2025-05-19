// Last updated: 19.05.2025, 09:14:20
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable();
        if nums[2] >= nums[0] + nums[1] {
            return "none".into();
        }
        if nums[2] == nums[0] {
            return "equilateral".into();
        }
        if nums[1] == nums[0] || nums[1] == nums[2] {
            return "isosceles".into();
        }
        "scalene".into()
    }
}