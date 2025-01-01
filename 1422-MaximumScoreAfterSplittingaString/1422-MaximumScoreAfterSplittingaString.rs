fn to_ones_and_zeros(balance: i32, count: usize) -> (usize, usize) {
    let zeros = (((count as i32) + balance) / 2) as usize;
    (zeros, count - zeros)
}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (balance, max_balance, max_balance_index) = s.as_bytes().iter().enumerate().fold(
            (0, i32::MIN, 0),
            |(mut balance, mut max_balance, mut max_balance_index), (index, &c)| {
                match c {
                    b'0' => balance += 1,
                    b'1' => balance -= 1,
                    _ => {}
                };
                if max_balance < balance && index < s.len() - 1 {
                    max_balance = balance;
                    max_balance_index = index;
                }
                (balance, max_balance, max_balance_index)
            },
        );
        let (left_zeros, left_ones) = to_ones_and_zeros(max_balance, max_balance_index + 1);
        let (_, total_ones) = to_ones_and_zeros(balance, s.len());
        (left_zeros + total_ones - left_ones) as i32
    }
}