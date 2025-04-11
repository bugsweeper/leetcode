// Last updated: 11.04.2025, 15:26:07
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut min_a, mut min_b) = (m, n);
        for op in ops {
            let [a, b] = op[..] else {
                unimplemented!()
            };
            min_a = min_a.min(a);
            min_b = min_b.min(b);
        }
        min_a * min_b
    }
}