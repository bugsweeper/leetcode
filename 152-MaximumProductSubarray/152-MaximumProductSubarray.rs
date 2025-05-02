// Last updated: 02.05.2025, 12:55:52
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = *nums.first().unwrap();
        let mut product = if max_product == 0 { 1 } else { max_product };
        let mut first_negative_product = if product > 0 { 1 } else { product };
        for num in nums.into_iter().skip(1) {
            if num == 0 {
                product = 1;
                first_negative_product = 1;
                max_product = max_product.max(0);
            } else {
                product *= num;
                if product > 0 {
                    max_product = max_product.max(product);
                } else {
                    max_product = max_product.max(product / first_negative_product);
                }
                if product < 0 && first_negative_product == 1 {
                    first_negative_product = product;
                }
            }
        }
        max_product
    }
}