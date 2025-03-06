impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|n| match (n % 3, n % 5) {
                (0, 0) => "FizzBuzz".into(),
                (0, _) => "Fizz".into(),
                (_, 0) => "Buzz".into(),
                _ => n.to_string(),
            })
            .collect::<Vec<_>>()
    }
}