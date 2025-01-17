const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn get_statistic(s: String) -> [i32; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &c in s.as_bytes() {
        *unsafe { statistic.get_unchecked_mut((c - b'a') as usize) } += 1;
    }
    statistic
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s_statistic = get_statistic(s);
        let t_statistic = get_statistic(t);
        s_statistic == t_statistic
    }
}