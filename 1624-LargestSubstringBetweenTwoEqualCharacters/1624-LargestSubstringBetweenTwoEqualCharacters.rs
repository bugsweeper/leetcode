// Last updated: 12.06.2025, 15:41:03
const SIZE: usize = b'z' as usize + 1;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut min_index = [None; SIZE];
        let mut max_index = [0; SIZE];
        for (index, byte) in s.bytes().enumerate() {
            let character = byte as usize;
            let index = index as i32;
            min_index[character].get_or_insert(index);
            max_index[character] = index;
        }
        min_index.into_iter().zip(max_index).filter_map(|(min, max)| {
            Some(max - min? - 1)
        }).max().unwrap()
    }
}