impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        match n {
            0 => vec![0],
            _ => {
                let n = n as usize;
                let mut result = Vec::with_capacity(n + 1);
                result.push(0);
                result.push(1);
                let mut power_of_2 = 2;
                while power_of_2 <= n {
                    for i in power_of_2..(power_of_2 << 1).min(n + 1) {
                        result.push(result[i - power_of_2] + 1);
                    }
                    power_of_2 <<= 1;
                }
                result
            }
        }
    }
}