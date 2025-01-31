use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

fn longest_path(
    start_node: i32,
    connections: &Vec<HashSet<i32>>,
    visited: &mut HashSet<i32>,
) -> (HashMap<i32, i32>, i32) {
    let mut queue = VecDeque::from([(start_node, 1)]); // (node, group)
    let mut distances = HashMap::from([(start_node, 1)]);
    while let Some((node, length)) = queue.pop_front() {
        for &neighbor in &connections[node as usize] {
            match distances.entry(neighbor) {
                Entry::Occupied(entry) => {
                    if *entry.get() == length {
                        return (HashMap::new(), -1);
                    }
                    continue;
                }
                _ => {}
            }
            queue.push_back((neighbor, length + 1));
            distances.insert(neighbor, length + 1);
            visited.insert(neighbor);
        }
    }
    let length = *distances.values().max().unwrap_or(&1);
    (distances, length)
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut connections = vec![std::collections::HashSet::new(); (n + 1) as usize];
        for edge in edges {
            unsafe {
                let (node1, node2) = (*edge.get_unchecked(0), *edge.get_unchecked(1));
                connections.get_unchecked_mut(node1 as usize).insert(node2);
                connections.get_unchecked_mut(node2 as usize).insert(node1);
            }
        }
        let mut visited = HashSet::new();
        let mut result = 0;
        for i in 1..=n {
            if visited.contains(&i) {
                continue;
            }

            visited.insert(i);
            let (component, length) = longest_path(i, &connections, &mut visited);
            if length == -1 {
                return -1;
            }

            let mut max_groups = length;
            for (node, _) in component {
                let (_, length) = longest_path(node, &connections, &mut visited);
                max_groups = max_groups.max(length);
            }
            result += max_groups;
        }
        result
    }
}