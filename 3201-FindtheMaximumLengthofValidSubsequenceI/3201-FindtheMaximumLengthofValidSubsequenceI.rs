// Last updated: 16.07.2025, 09:19:46
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let prev_alternating = *nums.first().unwrap() & 1;
        let (odds, evens, alternatings, _) = nums
            .into_iter()
            .fold((0, 0, 1, prev_alternating), |(mut odds, mut evens, mut alternatings, prev_alternating), num| {
                let cur = num & 1;
                if cur == 1 {
                    odds += 1;
                } else {
                    evens += 1;
                }
                if cur != prev_alternating {
                    alternatings += 1;
                }
                (odds, evens, alternatings, cur)
            });
        odds.max(evens).max(alternatings)
    }
}