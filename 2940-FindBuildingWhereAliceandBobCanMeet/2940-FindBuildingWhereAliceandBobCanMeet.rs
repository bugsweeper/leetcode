struct BinaryTreeSearcher {
    heights: Vec<i32>,
    // tree layout is like in priority queue of binary heap
    max_heights: Vec<(i32, usize)>,
    bottom_row_displacement: usize,
}

impl BinaryTreeSearcher {
    fn new(heights: Vec<i32>) -> Self {
        Self {
            heights,
            max_heights: Vec::new(),
            bottom_row_displacement: 0,
        }
    }
    fn leftmost_bigger(&mut self, mut alice: i32, mut bob: i32) -> i32 {
        if bob < alice {
            std::mem::swap(&mut alice, &mut bob);
        }
        if bob == alice || self.heights[alice as usize] < self.heights[bob as usize] {
            return bob;
        }
        if bob as usize == self.heights.len() - 1 {
            return -1;
        }
        if self.max_heights.is_empty() {
            let log2 = usize::BITS - self.heights.len().leading_zeros();
            let size = (1 << log2) as usize - 1 - (((1 << log2) - self.heights.len()) / 2);
            self.max_heights = vec![(i32::MIN, usize::MAX); size];
            self.bottom_row_displacement = (1 << (log2 - 1)) as usize - 1;
            for (index, slice) in self.heights.chunks(2).enumerate() {
                self.max_heights[self.bottom_row_displacement + index] =
                    if let &[first, second] = slice {
                        if first > second {
                            (first, index * 2)
                        } else {
                            (second, index * 2 + 1)
                        }
                    } else {
                        (slice[0], index * 2)
                    };
            }
            for index in (1..self.max_heights.len()).rev() {
                let upper_index = (index - 1) / 2;
                if self.max_heights[upper_index].0 < self.max_heights[index].0 {
                    self.max_heights[upper_index] = self.max_heights[index];
                }
            }
        }
        // Going up
        let bob = bob as usize;
        let mut pos = self.bottom_row_displacement + (bob + 1) / 2;
        let height = self.heights[alice as usize];
        while pos > 0 && (self.max_heights[pos].0 <= height || self.max_heights[pos].1 < bob + 1) {
            pos /= 2;
        }
        if self.max_heights[pos].0 <= height || self.max_heights[pos].1 < bob + 1 {
            return -1;
        }
        // Going down
        while pos < self.bottom_row_displacement {
            pos = 2 * pos + 1;
            if self.max_heights[pos].0 <= height || self.max_heights[pos].1 < bob + 1 {
                pos += 1;
            }
        }
        let mut index = (pos - self.bottom_row_displacement) * 2;
        if self.heights[index] <= height || index < bob + 1 {
            index += 1;
        }
        index as i32
    }
}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut searcher = BinaryTreeSearcher::new(heights);
        queries
            .into_iter()
            .map(move |query| {
                if let [alice, bob] = &query[..] {
                    searcher.leftmost_bigger(*alice, *bob)
                } else {
                    -1
                }
            })
            .collect()
    }
}