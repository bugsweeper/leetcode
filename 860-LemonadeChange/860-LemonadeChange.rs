// Last updated: 06.05.2025, 14:08:33
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut cash5, mut cash10) = (0, 0);
        for bill in bills {
            match bill {
                5 => cash5 += 1,
                10 => {
                    if cash5 == 0 {
                        return false;
                    } else {
                        cash5 -= 1;
                        cash10 += 1;
                    }
                }
                20 => {
                    if cash10 == 0 {
                        if cash5 < 3 {
                            return false;
                        } else {
                            cash5 -= 3;
                        }
                    } else {
                        if cash5 == 0 {
                            return false;
                        } else {
                            cash5 -= 1;
                        }
                        cash10 -= 1;
                    }
                }
                _ => unimplemented!(),
            }
        }
        true
    }
}