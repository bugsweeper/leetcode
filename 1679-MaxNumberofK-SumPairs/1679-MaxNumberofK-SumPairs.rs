impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_count = std::collections::HashMap::with_capacity(nums.len());
        for num in nums {
            if num < k {
                *num_count.entry(num).or_insert(0) += 1;
            }
        }
        let mut result = 0;
        let mut seen = std::collections::HashSet::with_capacity(num_count.len());
        for (num1, count1) in &num_count {
            let num2 = k - *num1;
            if seen.insert(*num1) && seen.insert(num2) {
                if let Some(count2) = num_count.get(&num2) {
                    result += count1.min(count2);
                }
            }
        }
        if k % 2 == 0 {
            if let Some(count) = num_count.get(&(k / 2)) {
                result += count / 2;
            }
        }
        result
    }
}