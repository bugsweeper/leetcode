// Last updated: 29.03.2025, 23:34:54
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const MODULO: usize = 1_000_000_007;

fn binary_exponentiation(mut base: usize, mut exponent: usize) -> usize {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % MODULO;
        }
        base = (base * base) % MODULO;
        exponent >>= 1;
    }
    result
}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let prime_score = nums.iter().map(|&num| {
            let mut score = 0;
            let mut num = num;
            for factor in 2..num.isqrt() + 1 {
                if num % factor == 0 {
                    score += 1;
                    while num % factor == 0 {
                        num /= factor;
                    }
                }
            }
            if num > 1 {
                score += 1;
            }
            score
        }).collect::<Vec<_>>();
        let mut left = vec![-1; nums.len()];
        let mut right = vec![nums.len(); nums.len()];
        let mut stack = Vec::new();
        for i in 0..nums.len() {
            while !stack.is_empty() && prime_score[*stack.last().unwrap()] < prime_score[i] {
                let index = stack.pop().unwrap();
                right[index] = i;
            }
            if !stack.is_empty() {
                left[i] = *stack.last().unwrap() as i32;
            }
            stack.push(i);
        }
        let mut k = k as usize;
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(index, num)| {
                let possible_operations =
                    (index as i32 - left[index]) as usize * (right[index] - index);
                (num, possible_operations)
            })
            .collect::<BinaryHeap<_>>();
        let mut result = 1;
        let (mut prev_num, mut prev_operations) = nums.pop().unwrap();
        while let Some((num, possible_operations)) = nums.pop() {
            if num == prev_num {
                prev_operations += possible_operations;
            } else {
                let operations = k.min(prev_operations);
                k -= operations;
                result = result * binary_exponentiation(prev_num as usize, operations) % MODULO;
                (prev_num, prev_operations) = (num, possible_operations)
            }
        }
        let operations = k.min(prev_operations);
        result = result * binary_exponentiation(prev_num as usize, operations) % MODULO;
        result as i32
    }
}