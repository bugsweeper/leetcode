// Last updated: 11.04.2025, 09:45:57
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut size = citations.len();
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if (citations[middle] as usize) < citations.len() - middle {
                base = middle;
            }
            size -= half;
        }
        (if (citations[base] as usize) < citations.len() - base {
            citations.len() - base - 1
        } else {
            citations.len() - base
        }) as i32
    }
}