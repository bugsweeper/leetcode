// Last updated: 01.10.2025, 07:45:36
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_bottles = num_bottles;
        let mut sum = num_bottles;
        while num_bottles >= num_exchange {
            sum += num_bottles / num_exchange;
            num_bottles = num_bottles / num_exchange + num_bottles % num_exchange;
        }
        sum
    }
}