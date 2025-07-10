// Last updated: 10.07.2025, 11:54:58
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::once;

#[derive(Clone, Copy, Eq)]
struct FreeSpan {
    start: i32,
    len: i32,
}

impl FreeSpan {
    fn new(start: i32, len: i32) -> Self {
        Self { start, len }
    }
}

impl Ord for FreeSpan {
    fn cmp(&self, other: &Self) -> Ordering {
        other.len.cmp(&self.len)
    }
}

impl PartialOrd for FreeSpan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FreeSpan {
    fn eq(&self, other: &Self) -> bool {
        self.len.eq(&other.len)
    }
}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let last_end = *end_time.last().unwrap();
        let free_spans = once(FreeSpan::new(0, start_time[0]))
            .chain(
                end_time
                    .iter()
                    .zip(start_time.iter().skip(1))
                    .map(|(end, start)| FreeSpan::new(*end, *start - *end)),
            )
            .chain(once(FreeSpan::new(last_end, event_time - last_end)))
            .collect::<Vec<_>>();
        // You might think that usage of BinaryHeap here has 0(n*log(n)) complexity
        // but I keep only top 3 free spans (by removing 4th least every time)
        // so complexity is O(n*log(4)) which is closer to O(n) and much better than sorting
        let mut top_free_spans = BinaryHeap::with_capacity(4);
        top_free_spans.extend(free_spans.iter().copied().take(3));
        for span in free_spans.iter().skip(3) {
            top_free_spans.push(*span);
            top_free_spans.pop();
        }
        let mut top_3_max_free_spans = Vec::with_capacity(3);
        while let Some(free_span) = top_free_spans.pop() {
            top_3_max_free_spans.push(free_span);
        }
        let mut max_free_time = 0;
        for pair in free_spans.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            let meeting_len = right.start - (left.start + left.len);
            let mut can_move = false;
            for &move_place in top_3_max_free_spans.iter().rev() {
                if meeting_len > move_place.len {
                    // meeting is too long
                    break;
                }
                if left.start != move_place.start && right.start != move_place.start {
                    can_move = true;
                    break;
                }
            }
            let free_time = if can_move {
                right.start + right.len - left.start
            } else {
                left.len + right.len
            };
            max_free_time = max_free_time.max(free_time);
        }
        max_free_time
    }
}