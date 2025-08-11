// Last updated: 11.08.2025, 13:19:46
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn count(s: String) -> [i32; ABC_LEN] {
    let mut count = [0; ABC_LEN];
    for byte in s.bytes() {
        count[(byte - b'a') as usize] += 1;
    }
    count
}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        count(s)
            .into_iter()
            .zip(count(target))
            .filter_map(|(available, target)| if target == 0 {None} else {Some(available / target)})
            .min()
            .unwrap()
    }
}