// Last updated: 01.08.2025, 11:23:32
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut odds = nums.iter().copied().skip(1).step_by(2).collect::<Vec<_>>();
        odds.sort_unstable_by_key(|num| -num);
        for (source, destination) in odds.into_iter().zip(nums.iter_mut().skip(1).step_by(2)) {
            *destination = source;
        }
        let mut evens = nums.iter().copied().step_by(2).collect::<Vec<_>>();
        evens.sort_unstable();
        for (source, destination) in evens.into_iter().zip(nums.iter_mut().step_by(2)) {
            *destination = source;
        }
        nums
    }
}