// Last updated: 18.04.2025, 13:20:16
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut least_tails: Vec<i32> = Vec::with_capacity(nums.len());
        for num in nums {
            let index_in_sequence = least_tails.partition_point(|&least_tail| least_tail < num);
            if let Some(tail) = least_tails.get_mut(index_in_sequence) {
                *tail = num.min(*tail);
            } else {
                least_tails.push(num);
            }
        }
        least_tails.len() as i32
    }
}
