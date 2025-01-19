use std::cmp::Ordering;

struct Cell {
    height: i32,
    i: usize,
    j: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.height.eq(&other.height)
    }
}

impl Eq for Cell {}

const VISITED: i32 = i32::MIN;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        if m < 3 {
            return 0;
        }
        let n = unsafe{height_map.get_unchecked(0)}.len();
        if n < 3 {
            return 0;
        }
        let mut priority_queue = std::collections::BinaryHeap::new();
        let mut height_map = height_map;
        for i in [0, m - 1] {
            let row = unsafe{height_map.get_unchecked_mut(i)};
            // corners have no impact on result
            *unsafe{row.get_unchecked_mut(0)} = VISITED;
            *unsafe{row.get_unchecked_mut(n - 1)} = VISITED;
            for j in 1..n - 1 {
                let height = unsafe{row.get_unchecked_mut (j)};
                priority_queue.push(Cell{i, j, height: *height});
                *height = VISITED;
            }
        }
        for i in 1..m - 1 {
            let row = unsafe{height_map.get_unchecked_mut(i)};
            for j in [0, n - 1] {
                let height = unsafe{row.get_unchecked_mut (j)};
                priority_queue.push(Cell{i, j, height: *height});
                *height = VISITED;
            }
        }
        let mut result = 0;
        let mut max_seen_height = i32::MIN; // Reassign by first value
        // The main idea that we are peaking visited cells from priority_queue from min to higher
        // continuously increasing max_seen_height, and adding all unvisited neighbors,
        // if there is gap between them with water, than we unexpectedly peak value that is less
        // than max_seen_height, so we add their diff to result
        while let Some(Cell{i, j, height}) = priority_queue.pop() {
            match height.cmp(&max_seen_height) {
                Ordering::Equal => {},
                Ordering::Greater => max_seen_height = height,
                Ordering::Less => result += max_seen_height - height,
            }
            if i > 0 {
                let neighbor_height = unsafe{height_map.get_unchecked_mut(i - 1).get_unchecked_mut(j)};
                if *neighbor_height != VISITED {
                    priority_queue.push(Cell{i: i - 1, j, height: *neighbor_height});
                    *neighbor_height = VISITED;
                }
            }
            if i < m - 1 {
                let neighbor_height = unsafe{height_map.get_unchecked_mut(i + 1).get_unchecked_mut(j)};
                if *neighbor_height != VISITED {
                    priority_queue.push(Cell{i: i + 1, j, height: *neighbor_height});
                    *neighbor_height = VISITED;
                }
            }
            let row = unsafe{height_map.get_unchecked_mut(i)};
            if j > 0 {
                let neighbor_height = unsafe{row.get_unchecked_mut(j - 1)};
                if *neighbor_height != VISITED {
                    priority_queue.push(Cell{i, j: j - 1, height: *neighbor_height});
                    *neighbor_height = VISITED;
                }
            }
            if j < n - 1 {
                let neighbor_height = unsafe{row.get_unchecked_mut(j + 1)};
                if *neighbor_height != VISITED {
                    priority_queue.push(Cell{i, j: j + 1, height: *neighbor_height});
                    *neighbor_height = VISITED;
                }
            }
        }
        result
    }
}