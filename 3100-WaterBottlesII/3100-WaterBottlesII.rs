// Last updated: 02.10.2025, 07:36:34
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut empty_bottles = num_bottles;
        let diff = num_bottles - num_exchange;
        let mut num_exchange = num_exchange;
        while empty_bottles >= num_exchange {
            empty_bottles -= num_exchange - 1;
            num_exchange += 1;
        }
        num_exchange + diff
    }
}