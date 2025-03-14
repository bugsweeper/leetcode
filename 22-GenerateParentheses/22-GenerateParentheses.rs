enum Direction {
    Forward,
    Backward,
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // State machine + greedy + backtracking
        let n = n as usize;
        let mut direction = Direction::Forward;
        let mut index = n;
        let mut open_count = n;
        let mut parentheses = vec![b'('; 2 * n];
        let mut result = Vec::with_capacity(1 << n);
        while index > 0 {
            match direction {
                Direction::Forward => {
                    if index == 2 * n {
                        direction = Direction::Backward;
                        result.push(String::from_utf8(parentheses.clone()).unwrap());
                        index -= 1;
                        continue;
                    }
                    parentheses[index] = if open_count < n {
                        open_count += 1;
                        b'('
                    } else {
                        b')'
                    };
                    index += 1;
                }
                Direction::Backward => match parentheses[index] {
                    b'(' => {
                        let close_count = index + 1 - open_count;
                        if close_count < open_count - 1 {
                            parentheses[index] = b')';
                            direction = Direction::Forward;
                            index += 1;
                        } else {
                            index -= 1;
                        }
                        open_count -= 1;
                    }
                    b')' => index -= 1,
                    _ => unimplemented!(),
                },
            }
        }
        result
    }
}