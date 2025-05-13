// Last updated: 13.05.2025, 16:58:26
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let str_len = strs[0].len();
        let mut deleted = 0;
        for j in 0..str_len {
            let mut prev = strs[0].as_bytes()[j];
            for row in strs.iter().skip(1) {
                let cur = row.as_bytes()[j];
                if cur < prev {
                    deleted += 1;
                    break;
                }
                prev = cur;
            }
        }
        deleted
    }
}