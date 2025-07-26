// Last updated: 26.07.2025, 22:43:54
use std::cmp::Ordering;

#[inline]
fn update_starts(start1: &mut i64, start2: &mut i64, start2_doubled: &mut bool, new_start: i64) -> bool {
    match new_start.cmp(start2) {
        Ordering::Equal => *start2_doubled = true,
        Ordering::Greater => (*start1, *start2, *start2_doubled) = (*start2, new_start, false),
        Ordering::Less => {if new_start > *start1 { *start1 = new_start }; return false}
    }
    true
}

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        if conflicting_pairs.len() == 1 {
            return (n * (n + 1)) >> 1;
        }
        let mut pairs = Vec::with_capacity(conflicting_pairs.len() + 1);
        pairs.extend(conflicting_pairs.into_iter().map(|pair| {
            let (p1, p2) = (pair[0] as i64, pair[1] as i64);
            if p1 > p2 {
                (p2, p1)
            } else {
                (p1, p2)
            }
        }));
        pairs.push((0, n + 1));
        // Reverse order for using pop()
        pairs.sort_unstable_by_key(|(_, end)| -*end);
        // Keeping only last two starts: from last start till current cell are 100% valid subarrays,
        // starting between start1 and start2 till current cell are valid subarrays if remove pair which starts from start2
        // only if start 2 is unique (keep checking doubling start 2)
        let (mut start1, mut start2, mut start2_doubled) = (0, 0, false);
        let first_end = pairs.last().unwrap().1;
        // before first pair all subarrays are 100% valid
        let (mut max_bonus, mut bonus, mut subarrays) = (0, 0, (first_end * (first_end - 1)) >> 1);
        while let Some((start, end)) = pairs.pop() && end != n + 1 {
            let mut switch_bonus = update_starts(&mut start1, &mut start2, &mut start2_doubled, start);
            while let Some((start, _)) = pairs.pop_if(|(_, next_end)| *next_end == end) {
                switch_bonus |= update_starts(&mut start1, &mut start2, &mut start2_doubled, start);
            }
            if switch_bonus {
                max_bonus = max_bonus.max(bonus);
                bonus = 0;
            }
            let next_end = pairs.last().unwrap().1;
            // 100% valid
            subarrays += ((next_end - end) * (end - start2 + next_end - 1 - start2)) >> 1;
            if !start2_doubled {
                // valid for most fresh started pair removal
                bonus += (next_end - end) * (start2 - start1);
            }
        }
        subarrays + max_bonus.max(bonus)
    }
}