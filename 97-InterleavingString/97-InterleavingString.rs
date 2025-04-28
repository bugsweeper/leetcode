// Last updated: 28.04.2025, 13:04:33
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

fn get_statistic(s: &[u8]) -> [usize; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &byte in s {
        statistic[(byte - b'a') as usize] += 1;
    }
    statistic
}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let statistic1 = get_statistic(s1);
        let statistic2 = get_statistic(s2);
        let statistic3 = get_statistic(s3);
        if statistic1.into_iter().zip(statistic2).zip(statistic3).any(|((s1, s2), s3)| s1 + s2 != s3) {
            return false;
        }
        let mut state = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        state[0][0] = true;
        for (index3, &byte3) in s3.iter().enumerate() {
            let mut at_least_one = false;
            let min_index1 = if index3 > s2.len() {index3 - s2.len()} else {0};
            for (index1, &byte1) in s1.iter().enumerate().take(index3 + 1).skip(min_index1) {
                if byte1 != byte3 {
                    continue;
                }
                let prev = state[index1][index3 - index1];
                state[index1 + 1][index3 - index1] |= prev;
                at_least_one |= prev;
            }
            let min_index2 = if index3 > s1.len() {index3 - s1.len()} else {0};
            for (index2, &byte2) in s2.iter().enumerate().take(index3 + 1).skip(min_index2) {
                if byte2 != byte3 {
                    continue;
                }
                let row = state.get_mut(index3 - index2).unwrap();
                let prev = row[index2];
                row[index2 + 1] |= prev;
                at_least_one |= prev;
            }
            if !at_least_one {
                return false;
            }
        }
        true
    }
}