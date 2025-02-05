impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut iter = s1
            .iter()
            .enumerate()
            .zip(s2.iter())
            .filter_map(|((index, &c1), &c2)| if c1 != c2 { Some(index) } else { None });
        match (iter.next(), iter.next(), iter.next()) {
            (None, None, None) => true,
            (Some(index1), Some(index2), None) => s1[index1] == s2[index2] && s1[index2] == s2[index1],
            _ => false,
        }
    }
}