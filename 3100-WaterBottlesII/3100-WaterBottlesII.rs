// Last updated: 02.10.2025, 07:31:53
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut empty_bottles = num_bottles;
        let mut bottles_drunk = num_bottles;
        let mut num_exchange = num_exchange;
        while empty_bottles >= num_exchange {
            bottles_drunk += 1;
            empty_bottles -= num_exchange - 1;
            num_exchange += 1;
        }
        bottles_drunk
    }
}