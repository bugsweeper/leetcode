impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity((tokens.len() + 1) / 2);
        for token in tokens {
            let value = match token.as_str() {
                "+" => stack.pop().unwrap() + stack.pop().unwrap(),
                "*" => stack.pop().unwrap() * stack.pop().unwrap(),
                "-" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    a - b
                },
                "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    a / b
                },
                _ => token.parse::<i32>().unwrap(),
            };
            stack.push(value)
        }
        stack.pop().unwrap()
    }
}