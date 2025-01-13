const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut statistic = [0; ABC_LEN];
        for &c in s.as_bytes() {
            statistic[(c - b'a') as usize] += 1;
        }
        statistic.into_iter().map(|count| match count {
            0..=2 => count,
            even if even % 2 == 0 => 2,
            odd if odd % 2 == 1 => 1,
            _ => 0,
        }).sum()
    }
}