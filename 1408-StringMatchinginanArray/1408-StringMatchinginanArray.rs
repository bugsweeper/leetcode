impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_unstable_by_key(|word| word.len());
        let mut answer = Vec::new();
        let mut current_index = 0;
        let mut current_len = unsafe { words.get_unchecked(current_index) }.len();
        let mut next_index = words.partition_point(|word| word.len() == current_len);
        while next_index != words.len() {
            for low_len in current_index..next_index {
                for higher_len in next_index..words.len() {
                    // Safety: both current_index and next_index are less than words.len()
                    if unsafe { words.get_unchecked(higher_len) }
                        .contains(unsafe { words.get_unchecked(low_len) })
                    {
                        let mut replacement = String::new();
                        // replace by empty string, because we don't need this word in array any more
                        std::mem::swap(&mut replacement, unsafe {
                            words.get_unchecked_mut(low_len)
                        });
                        answer.push(replacement);
                        break;
                    }
                }
            }
            current_index = next_index;
            current_len = unsafe { words.get_unchecked(current_index) }.len();
            next_index = words[current_index..].partition_point(|word| word.len() == current_len)
                + current_index;
        }
        answer
    }
}