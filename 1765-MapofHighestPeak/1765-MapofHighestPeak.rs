const UNVISITED: i32 = i32::MIN;
const WAS_WATER: i32 = 1;
const IS_WATER: i32 = 0;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut heights = is_water;
        let mut queue = std::collections::VecDeque::new();
        for (i, row) in heights.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if *cell == WAS_WATER {
                    queue.push_back((i, j, IS_WATER));
                    *cell = IS_WATER;
                } else {
                    *cell = UNVISITED;
                }
            }
        }
        while let Some((i, j, mut height)) = queue.pop_front() {
            height += 1;
            if i > 0 {
                let cell = unsafe { heights.get_unchecked_mut(i - 1).get_unchecked_mut(j) };
                if *cell == UNVISITED {
                    queue.push_back((i - 1, j, height));
                    *cell = height;
                }
            }
            if i < heights.len() - 1 {
                let cell = unsafe { heights.get_unchecked_mut(i + 1).get_unchecked_mut(j) };
                if *cell == UNVISITED {
                    queue.push_back((i + 1, j, height));
                    *cell = height;
                }
            }
            let row = unsafe { heights.get_unchecked_mut(i) };
            if j > 0 {
                let cell = unsafe { row.get_unchecked_mut(j - 1) };
                if *cell == UNVISITED {
                    queue.push_back((i, j - 1, height));
                    *cell = height;
                }
            }
            if j < row.len() - 1 {
                let cell = unsafe { row.get_unchecked_mut(j + 1) };
                if *cell == UNVISITED {
                    queue.push_back((i, j + 1, height));
                    *cell = height;
                }
            }
        }
        heights
    }
}