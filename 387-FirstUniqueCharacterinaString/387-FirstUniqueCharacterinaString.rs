const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut statistic = [(0, 0); ABC_LEN];
        for (index, &c) in s.as_bytes().iter().enumerate() {
            let cell = unsafe { statistic.get_unchecked_mut((c - b'a') as usize) };
            if cell.0 == 0 {
                cell.1 = index;
            }
            cell.0 += 1;
        }
        statistic
            .into_iter()
            .filter_map(|(count, first_index)| {
                if count == 1 {
                    Some(first_index as i32)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(-1)
    }
}