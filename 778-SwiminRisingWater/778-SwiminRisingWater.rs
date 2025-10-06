// Last updated: 06.10.2025, 15:06:02
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Cell {
    height: i32,
    i: usize,
    j: usize,
}

impl Cell {
    fn new(i: usize, j: usize, height: i32) -> Cell {
        Cell { height, i, j }
    }
}

impl Eq for Cell {}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
fn check_in_grid(
    i: usize,
    j: usize,
    unseen: &mut [Vec<bool>],
    grid: &[Vec<i32>],
    queue: &mut BinaryHeap<Cell>,
) {
    check_in_row(i, j, &mut unseen[i], &grid[i], queue);
}

#[inline]
fn check_in_row(
    i: usize,
    j: usize,
    unseen: &mut [bool],
    row: &[i32],
    queue: &mut BinaryHeap<Cell>,
) {
    let unseen = &mut unseen[j];
    if *unseen {
        *unseen = false;
        queue.push(Cell::new(i, j, row[j]));
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let last = grid.len() - 1;
        let mut max_height = grid[0][0];
        if last == 0 {
            return max_height;
        }
        let mut queue = BinaryHeap::with_capacity(grid.len() * 2);
        queue.push(Cell::new(0, 0, max_height));
        let mut unseen = vec![vec![true; grid.len()]; grid.len()];
        unseen[0][0] = false;
        while let Some(Cell { i, j, height }) = queue.pop() {
            max_height = max_height.max(height);
            if i == last && j == last {
                return max_height;
            }
            if i > 0 {
                check_in_grid(i - 1, j, &mut unseen, &grid, &mut queue);
            }
            if i < last {
                check_in_grid(i + 1, j, &mut unseen, &grid, &mut queue);
            }
            let (unseen, row) = (&mut unseen[i], &grid[i]);
            if j > 0 {
                check_in_row(i, j - 1, unseen, row, &mut queue);
            }
            if j < last {
                check_in_row(i, j + 1, unseen, row, &mut queue);
            }
        }
        unimplemented!();
    }
}