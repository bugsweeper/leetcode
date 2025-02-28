impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let bytes1 = str1.as_bytes();
        let bytes2 = str2.as_bytes();
        let mut previous = vec![Vec::new(); bytes2.len() + 1];
        let mut current = vec![Vec::new(); bytes2.len() + 1];
        for j in 0..bytes2.len() {
            previous[j].extend_from_slice(&bytes2[j..]);
        }
        for i in (0..bytes1.len()).rev() {
            current[bytes2.len()].clear();
            current[bytes2.len()].extend_from_slice(&bytes1[i..]);
            for j in (0..bytes2.len()).rev() {
                if bytes1[i] == bytes2[j] {
                    let target = &mut current[j];
                    target.clear();
                    target.push(bytes1[i]);
                    target.extend_from_slice(&previous[j + 1]);
                } else if previous[j].len() < current[j + 1].len() {
                    let target = &mut current[j];
                    target.clear();
                    target.push(bytes1[i]);
                    target.extend_from_slice(&previous[j]);
                } else {
                    let source = &current[j + 1];
                    let mut target = Vec::with_capacity(1 + source.len());
                    target.push(bytes2[j]);
                    target.extend_from_slice(source);
                    std::mem::swap(&mut current[j], &mut target);
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut previous[0]);
        String::from_utf8(result).unwrap()
    }
}