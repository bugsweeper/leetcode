// Last updated: 16.06.2025, 11:09:55
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut concatenated = String::with_capacity(1000);
        for word in word1 {
            concatenated.push_str(word.as_str());
        }
        let mut shift = 0;
        for word in word2 {
            if shift + word.len() > concatenated.len() {
                return false;
            }
            let piece = &concatenated[shift..shift + word.len()];
            if piece != &word {
                return false;
            }
            shift += word.len();
        }
        shift == concatenated.len()
    }
}