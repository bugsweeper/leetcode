impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = &strs[0][..];
        for item in strs.iter().skip(1) {
            if let Some(position) = prefix.chars().zip(item.chars()).position(|(c1, c2)| c1 != c2) {
                if position == 0 {
                    return String::new();
                }
                prefix = &prefix[..position];
            } else if prefix.len() > item.len() {
                prefix = &item[..];
            }
        }
        return prefix.into()
    }
}