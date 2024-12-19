impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (index, &item) in arr.iter().enumerate() {
            let item = item as usize;
            if item < start {
                while let Some(prev_start) = chunks.pop() {
                    if prev_start <= item {
                        start = prev_start;
                        break;
                    }
                }
            }
            if index > end {
                chunks.push(start);
                start = index;
                end = index;
            }
            end = end.max(item);
            if end == arr.len() - 1 {
                return chunks.len() as i32 + 1;
            }
        }
        chunks.len() as i32 + 1
    }
}