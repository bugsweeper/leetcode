// Last updated: 01.09.2025, 10:49:51
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct ClassRate {
    fail: usize,
    total: usize,
    divider: usize,
}

impl From<Vec<i32>> for ClassRate {
    fn from(value: Vec<i32>) -> Self {
        let total = value[1] as usize;
        let fail = total - value[0] as usize;
        let divider = total * (total + 1);
        ClassRate {
            fail,
            total,
            divider,
        }
    }
}

impl From<ClassRate> for f64 {
    fn from(value: ClassRate) -> Self {
        (value.total - value.fail) as f64 / value.total as f64
    }
}

impl Ord for ClassRate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.fail * other.divider).cmp(&(other.fail * self.divider))
    }
}

impl PartialOrd for ClassRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ClassRate {
    fn eq(&self, other: &Self) -> bool {
        self.fail.eq(&other.fail) && self.total.eq(&other.total)
    }
}

impl Eq for ClassRate {}

impl ClassRate {
    fn increment(&mut self) {
        self.total += 1;
        self.divider = self.total * (self.total + 1);
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = classes
            .into_iter()
            .map(ClassRate::from)
            .collect::<BinaryHeap<ClassRate>>();
        for _ in 0..extra_students {
            heap.peek_mut().unwrap().increment();
        }
        let total = heap.len();
        heap.into_iter().map(f64::from).sum::<f64>() / total as f64
    }
}