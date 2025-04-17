// Last updated: 17.04.2025, 09:47:21
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = 0;
        for (i, &numi) in nums.iter().enumerate() {
            for (j, &numj) in nums.iter().enumerate().skip(i + 1) {
                if numi == numj && (i * j) % k == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}