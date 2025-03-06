const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn get_statistic(s: String) -> [i32; ABC_LEN * 2] {
    let mut statistic = [0; ABC_LEN * 2];
    for &c in s.as_bytes() {
        let cell = unsafe {
            if c < b'a' {
                statistic.get_unchecked_mut((c - b'A') as usize)
            } else {
                statistic.get_unchecked_mut(ABC_LEN + (c - b'a') as usize)
            }
        };
        *cell += 1;
    }
    statistic
}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let (even_sum, has_odd) = get_statistic(s)
            .into_iter()
            .fold((0, false), |(even_sum, has_odd), count| {
                (even_sum + (count & !1), has_odd || count & 1 == 1)
            });
        even_sum + if has_odd { 1 } else { 0 }
    }
}