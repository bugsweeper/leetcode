// Last updated: 13.04.2025, 06:31:08
const MODULO: i64 = 1_000_000_007;

fn i_pow(mut base: i64, mut exponent: i64) -> i64 {
    let mut result = 1;
    loop {
        if exponent & 1 == 1 {
            result = result * base % MODULO;
        }
        exponent >>= 1;
        if exponent == 0 {
            break;
        }
        base = base * base % MODULO;
    }

    return result;
}

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let odd_power = n / 2;
        let even_power = odd_power + (n & 1);
        (i_pow(5, even_power) * i_pow(4, odd_power) % MODULO) as i32
    }
}