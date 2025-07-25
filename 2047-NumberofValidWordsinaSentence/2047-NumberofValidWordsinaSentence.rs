// Last updated: 25.07.2025, 15:11:56
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let rounds = tickets[k];
        let (head, tail) = tickets.split_at(k + 1);
        let mut time = head.iter()
            .map(|&ticket| ticket.min(rounds))
            .sum();
        time += match rounds {
            1 => 0,
            2 => tail.len() as i32,
            _ => tail.iter()
                .map(|&ticket| ticket.min(rounds - 1))
                .sum(),
        };
        time
    }
}