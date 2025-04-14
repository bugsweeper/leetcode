// Last updated: 14.04.2025, 08:33:47
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        for (i, num_i) in arr.iter().enumerate() {
            for (j, num_j) in arr.iter().enumerate().skip(i + 1) {
                if (num_i - num_j).abs() > a {
                    continue;
                }
                for (k, num_k) in arr.iter().enumerate().skip(j + 1) {
                    if (num_j - num_k).abs() <= b && (num_i - num_k).abs() <= c {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}