// Last updated: 19.05.2025, 15:22:08
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut defanged = String::with_capacity(address.len() + 8);
        for character in address.chars() {
            if character == '.' {
                defanged.push_str("[.]");
            } else {
                defanged.push(character);
            }
        }
        defanged
    }
}