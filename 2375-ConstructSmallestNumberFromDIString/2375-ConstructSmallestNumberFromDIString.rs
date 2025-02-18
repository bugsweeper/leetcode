impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let pattern = pattern.as_bytes();
        let mut result = Vec::with_capacity(pattern.len() + 1);
        result.push(b'1');
        let mut first_moving_index = 0;
        let mut previous_direction = pattern[0];
        let mut lower_level = b'1';
        let mut higher_level = b'1';
        for (index, &direction) in pattern.iter().enumerate() {
            higher_level += 1;
            if previous_direction != direction {
                first_moving_index = index;
            }
            match direction {
                b'D' => {
                    if previous_direction == b'I' {
                        lower_level = higher_level - 1;
                    }
                    result.push(lower_level);
                    for num in &mut result[first_moving_index..=index] {
                        *num += 1;
                    }
                },
                b'I' => {
                    result.push(higher_level)
                },
                _ => {},
            }
            previous_direction = direction;
        }
        unsafe{String::from_utf8_unchecked(result)}
    }
}