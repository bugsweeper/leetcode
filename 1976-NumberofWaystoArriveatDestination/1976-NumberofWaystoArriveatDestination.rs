// Last updated: 23.03.2025, 15:53:58
use std::cmp::{Ordering, Reverse};

const MODULO: usize = 1_000_000_007;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut connections = vec![vec![]; n];
        for road in roads {
            let [u, v, w] = road[..] else {
                continue;
            };
            let (u, v, w) = (u as usize, v as usize, w as usize);
            connections[u].push((w, v));
            connections[v].push((w, u));
        }
        let mut queue = std::collections::BinaryHeap::with_capacity(n);
        queue.push((Reverse(0), 0usize));
        let mut min_paths = vec![(usize::MAX, 0); n];
        min_paths[0].1 = 1;
        while let Some((Reverse(cost), node)) = queue.pop() {
            let path_count_to_current_node = min_paths[node].1;
            for &(neighbor_cost, neighbor_node) in &connections[node] {
                let new_cost_to_neighbor = cost + neighbor_cost;
                match new_cost_to_neighbor.cmp(&min_paths[neighbor_node].0) {
                    Ordering::Less => {
                        let min_neighbor_path = min_paths.get_mut(neighbor_node).unwrap();
                        *min_neighbor_path = (new_cost_to_neighbor, path_count_to_current_node);
                        queue.push((Reverse(new_cost_to_neighbor), neighbor_node));
                    }
                    Ordering::Equal => {
                        let min_neighbor_path = min_paths.get_mut(neighbor_node).unwrap();
                        min_neighbor_path.1 =
                            (min_neighbor_path.1 + path_count_to_current_node) % MODULO;
                    }
                    _ => {}
                }
            }
        }
        min_paths[n - 1].1 as i32
    }
}