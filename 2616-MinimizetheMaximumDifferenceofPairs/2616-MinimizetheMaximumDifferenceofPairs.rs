// Last updated: 13.06.2025, 18:19:26
fn answer_is_big_enough(diffs: &[i32], answer: i32, mut pairs: i32) -> bool {
    let mut index = 0;
    while index < diffs.len() {
        if diffs[index] <= answer {
            index += 2;
            pairs -= 1;
            if pairs == 0 {
                return true;
            }
        } else {
            index += 1;
        }
    }
    false
}

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }
        match nums.len() {
            1 => return 0,
            2 => return (nums[0] - nums[1]).abs(),
            _ => {},
        }
        let mut unsorted_diffs = nums.clone();
        unsorted_diffs.sort_unstable();
        let mut prev_diff = unsorted_diffs.pop().unwrap();
        for diff in unsorted_diffs.iter_mut().rev() {
            (prev_diff, *diff) = (*diff, prev_diff - *diff)
        }
        let mut diffs = unsorted_diffs.clone();
        diffs.sort_unstable();
        let mut unique_diffs = diffs.clone();
        unique_diffs.dedup();
        let index = unique_diffs.partition_point(|&diff| !answer_is_big_enough(&unsorted_diffs, diff, p));
        unique_diffs[index]
    }
}