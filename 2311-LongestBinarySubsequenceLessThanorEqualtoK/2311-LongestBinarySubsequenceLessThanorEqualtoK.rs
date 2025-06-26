// Last updated: 26.06.2025, 11:00:22
use std::cmp::Ordering;

#[inline(always)]
fn bits2number(bits: &[u8]) -> i32 {
    let mut number = 0;
    for &bit in bits {
        number <<= 1;
        if bit == b'1' {
            number += 1;
        }
    }
    number
}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let max_bits = k.ilog2() as usize + 1;
        (match s.len().cmp(&max_bits) {
            Ordering::Equal => {
                if bits2number(s.as_bytes()) <= k {
                    max_bits
                } else {
                    max_bits - 1
                }
            }
            Ordering::Greater => {
                let (count_zeros, check_overflow) = s.as_bytes().split_at(s.len() - max_bits);
                count_zeros
                    .iter()
                    .filter(|&maybe_zero| *maybe_zero == b'0')
                    .count()
                    + if bits2number(check_overflow) <= k {
                        max_bits
                    } else {
                        max_bits - 1
                    }
            }
            Ordering::Less => s.len(),
        }) as i32
    }
}