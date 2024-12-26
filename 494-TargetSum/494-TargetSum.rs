impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum_ways = std::collections::HashMap::<i32, i32>::new();
        sum_ways.insert(0, 1);
        for num in nums {
            let mut next_sum_ways = std::collections::HashMap::with_capacity(sum_ways.len());
            for (sum, ways) in sum_ways {
                *next_sum_ways.entry(sum + num).or_default() += ways;
                *next_sum_ways.entry(sum - num).or_default() += ways;
            }
            sum_ways = next_sum_ways;
        }
        *sum_ways.get(&target).unwrap_or(&0)
    }
}