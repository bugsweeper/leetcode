impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut left_zeroed_items = nums.iter().position(|num| *num > 0).unwrap_or(nums.len());
        if left_zeroed_items == nums.len() {
            return 0;
        }
        let mut query_diff_sum = vec![0; nums.len() + 1];

        for (index, query) in queries.into_iter().enumerate() {
            let &[l, r, val] = &query[..] else {
                unimplemented!();
            };
            let l = left_zeroed_items.max(l as usize);
            let r = r as usize + 1;
            if l >= r {
                continue;
            }
            query_diff_sum[l] += val;
            query_diff_sum[r] -= val;
            if l == left_zeroed_items {
                while left_zeroed_items < nums.len()
                    && query_diff_sum[left_zeroed_items] >= nums[left_zeroed_items]
                {
                    query_diff_sum[left_zeroed_items + 1] += query_diff_sum[left_zeroed_items];
                    left_zeroed_items += 1;
                }
            }
            if left_zeroed_items >= nums.len() {
                return index as i32 + 1;
            }
        }
        -1
    }
}