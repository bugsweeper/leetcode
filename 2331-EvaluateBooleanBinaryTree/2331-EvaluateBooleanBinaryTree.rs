// Last updated: 08.09.2025, 14:50:11
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();
        let [low, middle, high] = amount[..] else {
            unimplemented!();
        };
        if high > low + middle {
            return high
        }
        (low + middle + high + 1) >> 1
    }
}