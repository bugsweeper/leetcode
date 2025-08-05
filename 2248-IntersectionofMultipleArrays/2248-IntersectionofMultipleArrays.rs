// Last updated: 05.08.2025, 15:17:33
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        if let Some(digit) = num
            .as_bytes()
            .windows(3)
            .filter_map(|slice| {
                let &[d1, d2, d3] = slice else {
                    unimplemented!();
                };
                if d1 == d2 && d2 == d3 { Some(d1) } else { None }
            })
            .max()
        {
            let digit = digit as char;
            let mut answer = String::with_capacity(3);
            answer.push(digit);
            answer.push(digit);
            answer.push(digit);
            answer
        } else {
            String::new()
        }
    }
}