use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut products_frequency = HashMap::with_capacity(n * (n - 1) >> 1);
        let mut number_of_tuples = 0;
        for (i, &num1) in nums.iter().take(n - 1).enumerate() {
            for &num2 in nums.iter().skip(i + 1) {
                *products_frequency.entry(num1 * num2).or_insert(0) += 1;
            }
        }
        for &frequency in products_frequency.values() {
            number_of_tuples += (frequency * (frequency - 1)) << 2;
        }
        number_of_tuples
    }
}