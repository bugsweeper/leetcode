impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut unlocked_count = 0;
        let mut locked_balance: i32 = 0;
        for (&p, &l) in s.as_bytes().iter().zip(locked.as_bytes().iter()) {
            if l == b'0' {
                unlocked_count += 1;
            } else if p == b'(' {
                locked_balance += 1;
            } else {
                locked_balance -= 1;
            }
            if locked_balance < -unlocked_count {
                return false;
            }
        }
        unlocked_count = 0;
        locked_balance = 0;
        for (&p, &l) in s
            .as_bytes()
            .iter()
            .rev()
            .zip(locked.as_bytes().iter().rev())
        {
            if l == b'0' {
                unlocked_count += 1;
            } else if p == b'(' {
                locked_balance += 1;
            } else {
                locked_balance -= 1;
            }
            if locked_balance > unlocked_count {
                return false;
            }
        }
        true
    }
}