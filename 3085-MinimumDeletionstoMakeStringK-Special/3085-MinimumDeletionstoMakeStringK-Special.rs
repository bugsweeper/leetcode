// Last updated: 21.06.2025, 15:47:41
impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let k = k as usize;
        let mut freq = [0; b'z' as usize + 1];
        for byte in word.bytes() {
            freq[byte as usize] += 1;
        }
        let mut freq = freq[b'a' as usize..=b'z' as usize]
            .iter()
            .filter_map(|&count| if count == 0 { None } else { Some(count) })
            .collect::<Vec<_>>();
        freq.sort_unstable();
        let mut index = 0;
        let mut min_deletions = word.len();
        let mut deleted_letters = 0;
        let mut min_letters = freq[0];
        while index < freq.len() {
            let max_letters = min_letters + k;
            let mut deletions = deleted_letters;
            for &count in &freq[index..] {
                deletions += count.saturating_sub(max_letters);
            }
            min_deletions = min_deletions.min(deletions);
            for (i, &count) in freq[index..].iter().enumerate() {
                if count == min_letters {
                    deleted_letters += count;
                } else {
                    index += i;
                    min_letters = count;
                    break;
                }
            }
            if deleted_letters > min_deletions || deleted_letters == word.len() {
                break;
            }
        }
        min_deletions as i32
    }
}