// Last updated: 20.05.2025, 11:34:19
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut prefix_sum = vec![0; nums.len()];
        for query in queries {
            let [start, end] = query[..] else {
                unimplemented!("query is not a valid query");
            };
            prefix_sum[start as usize] += 1;
            let end = end as usize + 1;
            if end < prefix_sum.len() {
                prefix_sum[end] -= 1;
            }
        }
        let mut decreased = 0;
        for (num, prefix) in nums.into_iter().zip(prefix_sum) {
            decreased += prefix;
            if decreased < num {
                return false;
            }
        }
        true
    }
}