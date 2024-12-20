use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        s.iter()
            .map(|&symbol| roman_map[&(symbol as char)])
            .fold((0, 0), |(sum, previous_value), value| {
                (
                    sum + value
                        + if previous_value < value {
                            -2 * previous_value
                        } else {
                            0
                        },
                    value,
                )
            })
            .0
    }
}