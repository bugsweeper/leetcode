// Last updated: 19.06.2025, 13:51:30
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const UPPER_START: usize = b'A' as usize;
const LOWER_START: usize = b'a' as usize;

// I know there is alot of code, but i tried make it faster than 0(n^2) for larger strings
// Idea behind this is collect for each subslice (starting from string as single subslice)
// split points, then push splited subslices to priority queue (makes it greedy)
// Code could be twice less, but I tried to make `prev_indexes` vector, which tracks backward occurance
// of each letter (for fast searching split points) only once, that is why first run on all string
// is made in own code copy with small differences

pub fn longest_nice_substring(slice: &str) -> (usize, usize) {
    let mut queue = BinaryHeap::with_capacity(slice.len());
    let mut prev_indexes = Vec::with_capacity(slice.len());
    let mut last_index = vec![None; b'z' as usize + 1];
    for (index, byte) in slice.bytes().enumerate() {
        let byte_index = byte as usize;
        let prev_index = last_index[byte_index].replace(index);
        prev_indexes.push(prev_index);
    }
    let mut splits = Vec::with_capacity(slice.len());
    for (&upper_index, &lower_index) in last_index[UPPER_START..].iter().zip(&last_index[LOWER_START..]).take(ABC_LEN) {
        match (upper_index, lower_index) {
            (Some(mut index), None) | (None, Some(mut index)) => {
                splits.push(index);
                while let Some(prev_index) = prev_indexes[index] {
                    splits.push(prev_index);
                    index = prev_index;
                }
            }
            _ => {}
        }
    }
    if splits.is_empty() {
        return (0, slice.len());
    }
    splits.sort_unstable();
    let mut prev_index = 0;
    for split in splits {
        let len = split - prev_index;
        if len > 0 {
            queue.push((len, Reverse(prev_index)));
        }
        prev_index = split + 1;
    }
    let len = slice.len() - prev_index;
    if len > 0 {
        queue.push((len, Reverse(prev_index)));
    }
    while let Some((len, Reverse(start))) = queue.pop() {
        let mut last_index = vec![None; b'z' as usize + 1];
        for (index, byte) in slice.bytes().enumerate().skip(start).take(len) {
            let byte_index = byte as usize;
            last_index[byte_index] = Some(index);
        }
        let mut splits = Vec::with_capacity(len);
        for (&upper_index, &lower_index) in last_index[UPPER_START..].iter().zip(&last_index[LOWER_START..]).take(ABC_LEN) {
            match (upper_index, lower_index) {
                (Some(mut index), None) | (None, Some(mut index)) => {
                    splits.push(index);
                    while let Some(prev_index) = prev_indexes[index] {
                        if prev_index < start {
                            break;
                        }
                        splits.push(prev_index);
                        index = prev_index;
                    }
                }
                _ => {}
            }
        }
        if splits.is_empty() {
            return (start, len);
        }
        splits.sort_unstable();
        let mut prev_index = start;
        for split in splits {
            let subslice_len = split - prev_index;
            if len > 0 {
                queue.push((subslice_len, Reverse(prev_index)));
            }
            prev_index = split + 1;
        }
        let subslice_len = start + len - prev_index;
        if subslice_len > 0 {
            queue.push((subslice_len, Reverse(prev_index)));
        }
    }
    (0, 0)
}

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let (start, length) = longest_nice_substring(&s);
        s[start..start + length].into()
    }
}