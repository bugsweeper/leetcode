// Last updated: 21.03.2025, 16:33:18
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
    pub fn find_the_difference(s: String, t: String) -> char {
        for (index, (s, t)) in get_statistic(s)
            .into_iter()
            .zip(get_statistic(t).into_iter())
            .enumerate()
        {
            if s < t {
                return (index as u8 + b'a') as char;
            }
        }
        unimplemented!()
    }
}