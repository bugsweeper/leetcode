// Last updated: 13.10.2025, 00:02:48
use std::collections::HashMap;

const MODULO: i64 = 1_000_000_007;

const fn fast_power(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1;
    base %= MODULO;
    while exp != 0 {
        if exp & 1 == 1 {
            result = result * base % MODULO;
        }
        base = base * base % MODULO;
        exp >>= 1;
    }
    result
}

const MAX: usize = 55;
const FACTORIAL: [i64; MAX] = {
    let mut array = [1; MAX];
    let mut factorial = 1;
    let mut i = 2;
    while i < MAX {
        factorial = factorial * i as i64 % MODULO;
        array[i] = factorial;
        i += 1;
    }
    array
};

const INVERSE_FACTORIAL: [i64; MAX] = {
    let mut array = [1; MAX];
    let mut factorial = fast_power(FACTORIAL[MAX - 1], MODULO - 2);
    array[MAX - 1] = factorial;
    let mut i = MAX - 2;
    while i > 1 {
        factorial = factorial * (i + 1) as i64 % MODULO;
        array[i] = factorial;
        i -= 1;
    }
    array
};

fn recursive_dp(
    index: usize,
    nums: &[i32],
    m_left: i32,
    k_left: i32,
    carry: i32,
    dp: &mut HashMap<(usize, i32, i32, i32), i64>,
) -> i64 {
    if index == nums.len() {
        return if m_left == 0 && (k_left == 0 && carry == 0 || carry.count_ones() == k_left as u32)
        {
            1
        } else {
            0
        };
    }
    if let Some(&result) = dp.get(&(index, m_left, k_left, carry)) {
        return result;
    }
    let mut total = 0;
    for i in 0..=m_left {
        let bit_used = (i + carry) & 1;
        if k_left >= bit_used {
            total += recursive_dp(
                index + 1,
                nums,
                m_left - i,
                k_left - bit_used,
                (carry + i) >> 1,
                dp,
            ) * fast_power(nums[index] as i64, i as i64)
                % MODULO
                * INVERSE_FACTORIAL[i as usize]
                % MODULO;
            total %= MODULO;
        }
    }
    dp.insert((index, m_left, k_left, carry), total);
    total
}

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let mut dp = HashMap::with_capacity(nums.len() * (m * k) as usize * 15);
        (recursive_dp(0, &nums, m, k, 0, &mut dp) * FACTORIAL[m as usize] % MODULO) as i32
    }
}