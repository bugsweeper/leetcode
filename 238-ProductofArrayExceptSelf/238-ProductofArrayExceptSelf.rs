impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product = nums[0];
        let n = nums.len();
        let mut prefixes = Vec::with_capacity(n);
        prefixes.push(product);
        for i in 1..n-1 {
            product *= nums[i];
            prefixes.push(product);
        }
        product = nums[n - 1];
        let mut answer = nums;
        if n > 1 {
            answer[n - 1] = prefixes[n - 2];
        }
        for i in (1..(n - 1)).rev() {
            let value = answer[i];
            answer[i] = product * prefixes[i - 1];
            product *= value;
        }
        answer[0] = product;
        answer
    }
}