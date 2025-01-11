const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let mut statistic = [0; ABC_LEN];
        for &c in s.as_bytes() {
            let c = (c - b'a') as usize;
            *unsafe { statistic.get_unchecked_mut(c) } += 1;
        }
        statistic.into_iter().filter(|count| count % 2 == 1).count() <= k as usize
    }
}