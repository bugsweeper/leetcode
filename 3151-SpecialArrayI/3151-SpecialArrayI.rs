impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut previous = *unsafe { nums.get_unchecked(0) };
        for num in nums.into_iter().skip(1) {
            if num % 2 == previous % 2 {
                return false;
            }
            previous = num;
        }
        true
    }
}