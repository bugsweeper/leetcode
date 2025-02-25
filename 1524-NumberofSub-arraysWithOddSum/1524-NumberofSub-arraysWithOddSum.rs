const MODULO: i32 = 1_000_000_007;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut prefix_sum = Vec::with_capacity(arr.len());
        let mut sum = 0;
        for num in arr {
            sum += num;
            prefix_sum.push(sum & 1);
        }
        let mut prefix_even_count = 1;
        let mut prefix_odd_count = 0;
        let mut odd_count = 0;
        for i in 0..prefix_sum.len() {
            let current = prefix_sum[i];
            if current == 0 {
                odd_count = (odd_count + prefix_odd_count) % MODULO;
                prefix_even_count += 1;
            } else {
                odd_count = (odd_count + prefix_even_count) % MODULO;
                prefix_odd_count += 1;
            }
        }
        odd_count
    }
}