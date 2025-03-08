impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let bytes = blocks.as_bytes();
        let k = k as usize;
        let mut recolors = bytes.iter().take(k).filter(|block| **block == b'W').count();
        let mut min_recolors = recolors;
        for i in k..bytes.len() {
            if bytes[i] == b'W' {
                recolors += 1;
            }
            if bytes[i - k] == b'W' {
                recolors -= 1;
            }
            min_recolors = min_recolors.min(recolors)
        }
        min_recolors as i32
    }
}