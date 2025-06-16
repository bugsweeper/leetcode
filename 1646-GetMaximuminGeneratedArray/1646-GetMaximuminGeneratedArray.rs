// Last updated: 16.06.2025, 12:08:34
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut source = command.as_bytes();
        let mut result = String::with_capacity(command.len());
        while !source.is_empty() {
            if source[0] == b'G' {
                source = &source[1..];
                result.push('G');
                continue;
            }
            match source[1] {
                b')' => {
                    source = &source[2..];
                    result.push('o');
                }
                b'a' => {
                    source = &source[4..];
                    result.push_str("al");
                }
                _ => {}
            }
        }
        result
    }
}