// Last updated: 28.03.2025, 14:23:57
use std::cmp::Reverse;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let row_len = grid[0].len();
        let mut result = vec![(grid.len() * row_len) as i32; queries.len()];
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|(_, value)| *value);
        let mut query_iter = queries.into_iter();
        let Some((mut index, mut query_value)) = query_iter.next() else {
            unimplemented!();
        };
        let mut priority_queue = std::collections::BinaryHeap::with_capacity(grid.len() * 2);
        priority_queue.push((Reverse(grid[0][0]), 0, 0));
        let mut seen = vec![false; grid.len() * row_len];
        seen[0] = true;
        let mut grid_cell_count = 0;
        while let Some((Reverse(grid_value), i, j)) = priority_queue.pop() {
            while query_value <= grid_value {
                result[index] = grid_cell_count;
                if let Some((next_index, next_query)) = query_iter.next() {
                    (index, query_value) = (next_index, next_query);
                } else {
                    // No queries left
                    return result;
                }
            }
            grid_cell_count += 1;
            // This grid value is collected add neighbours
            if i > 0 {
                let seen = seen.get_mut((i - 1) * row_len + j).unwrap();
                if !*seen {
                    *seen = true;
                    priority_queue.push((Reverse(grid[i - 1][j]), i - 1, j));
                }
            }
            if i < grid.len() - 1 {
                let seen = seen.get_mut((i + 1) * row_len + j).unwrap();
                if !*seen {
                    *seen = true;
                    priority_queue.push((Reverse(grid[i + 1][j]), i + 1, j));
                }
            }
            let row = grid.get(i).unwrap();
            if j > 0 {
                let seen = seen.get_mut(i * row_len + j - 1).unwrap();
                if !*seen {
                    *seen = true;
                    priority_queue.push((Reverse(row[j - 1]), i, j - 1));
                }
            }
            if j < row_len - 1 {
                let seen = seen.get_mut(i * row_len + j + 1).unwrap();
                if !*seen {
                    *seen = true;
                    priority_queue.push((Reverse(row[j + 1]), i, j + 1));
                }
            }
        }
        result
    }
}
