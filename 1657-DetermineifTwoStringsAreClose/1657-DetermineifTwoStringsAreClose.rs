// Last updated: 18.04.2025, 13:38:26
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn get_statistic(s: &str) -> [i32; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &c in s.as_bytes() {
        statistic[(c - b'a') as usize] += 1;
    }
    statistic
}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut s1 = get_statistic(&word1);
        let mut s2 = get_statistic(&word2);
        if !s1
            .iter()
            .zip(s2.iter())
            .all(|(&c1, &c2)| (c1 == 0) == (c2 == 0))
        {
            return false;
        }
        s1.sort_unstable();
        s2.sort_unstable();
        s1 == s2
    }
}

