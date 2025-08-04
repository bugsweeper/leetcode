// Last updated: 04.08.2025, 16:41:35
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut round = s.into_bytes();
        let k = k as usize;
        for byte in &mut round {
            *byte -= b'0';
        }
        while round.len() > k {
            let mut next_round = Vec::with_capacity(round.len());
            for group in round.chunks(k) {
                let mut sum = 0;
                for &byte in group {
                    sum += byte as i32;
                }
                if sum >= 100 {
                    next_round.push((sum / 100) as u8);
                    sum %= 100;
                    next_round.push((sum / 10) as u8);
                    next_round.push((sum % 10) as u8);
                } else if sum >= 10 {
                    next_round.push((sum / 10) as u8);
                    next_round.push((sum % 10) as u8);
                } else {
                    next_round.push(sum as u8);
                }
            }
            std::mem::swap(&mut round, &mut next_round);
        }
        round
            .into_iter()
            .map(|byte| (byte + b'0') as char)
            .collect::<String>()
    }
}