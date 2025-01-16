const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[inline]
fn get_statistic(s: &str) -> [i32; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &c in s.as_bytes() {
        *unsafe{statistic.get_unchecked_mut((c - b'a') as usize)} += 1;
    }
    statistic
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: std::collections::HashMap<[i32; 26], Vec<_>> = std::collections::HashMap::with_capacity(strs.len());
        for item in strs {
            let key = get_statistic(&item);
            groups.entry(key).or_default().push(item);
        }
        groups.into_values().collect::<Vec<_>>()
    }
}