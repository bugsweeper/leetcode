impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let sum: i64 = candies.iter().map(|candy| *candy as i64).sum();
        if sum < k {
            return 0;
        }
        let mut base = 1;
        let mut size = sum / k;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            let mut count = 0;
            for candy in &candies {
                count += *candy as i64 / middle;
                if count >= k {
                    base = middle;
                    break;
                }
            }
            size -= half;
        }
        base as i32
    }
}