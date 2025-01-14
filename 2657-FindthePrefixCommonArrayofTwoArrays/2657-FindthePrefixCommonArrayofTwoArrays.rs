impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_bits: u64 = 0;
        let mut b_bits: u64 = 0;
        let mut result = Vec::with_capacity(a.len());
        for (a, b) in a.into_iter().zip(b.into_iter()) {
            a_bits |= 1 << a;
            b_bits |= 1 << b;
            result.push((a_bits & b_bits).count_ones() as i32);
        }
        result
    }
}