use std::collections::HashSet;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut processed_products = HashSet::with_capacity((n - 3) * (n + 1) / 2);
        let nums_set = nums.iter().copied().collect::<HashSet<_>>();
        let mut combinations = 0;
        for (i, &num1) in nums.iter().take(n-3).enumerate() {
            for &num2 in nums.iter().skip(i + 1) {
                let product = num1 * num2;
                if processed_products.contains(&product) {
                    continue;
                }
                let mut multipliers = 0;
                for &maybe_multiplier in nums.iter().skip(i) {
                    if product % maybe_multiplier == 0 {
                        let second_multiplier = product / maybe_multiplier;
                        if second_multiplier != maybe_multiplier && nums_set.contains(&(product / maybe_multiplier)) {
                            multipliers += 1;
                        }
                    }
                }
                if multipliers > 3 {
                    combinations += multipliers * ((multipliers - 2))
                }
                processed_products.insert(product);
            }
        }
        combinations
    }
}