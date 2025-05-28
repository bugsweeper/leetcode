// Last updated: 28.05.2025, 15:42:16
#[inline(always)]
fn edges2connections(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut connections = vec![vec![]; edges.len() + 1];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        connections[u].push(v);
        connections[v].push(u);
    }
    connections
}

#[inline(always)]
fn calculate_target_nodes(max_k: usize, connections: &[Vec<usize>]) -> Vec<i32> {
    if max_k == 0 {
        // With no edges all nodes are target nodes only for themselves
        return vec![1; connections.len()];
    } else if max_k >= connections.len() - 1 {
        return vec![connections.len() as i32; connections.len()];
    }
    // Every level of deepness for current node contains amount of nodes in some distance for every branch and sum of all branches + current node
    let mut prev_deepness_level = connections
        .iter()
        .map(|neighbors| {
            let mut neighbors_count = vec![0; connections.len() + 1];
            for &neighbor in neighbors.iter() {
                neighbors_count[neighbor] += 1;
            }
            *neighbors_count.last_mut().unwrap() = neighbors.len() as i32 + 1;
            neighbors_count
        })
        .collect::<Vec<_>>();
    let mut cur_deepness_level = vec![vec![0; connections.len() + 1]; connections.len()];
    for _ in 1..max_k {
        let mut complete_count = 0;
        for (node_index, (branches, neighbors)) in cur_deepness_level.iter_mut().zip(connections.iter()).enumerate() {
            let mut sum = 1;    // count self node
            for &neighbor_index in neighbors.iter() {
                let branch = &mut branches[neighbor_index];
                let prev_branch_node_deepness = &prev_deepness_level[neighbor_index];
                let exclude = prev_branch_node_deepness[node_index];
                *branch = *prev_branch_node_deepness.last().unwrap() - exclude;
                sum += *branch;
            }
            *branches.last_mut().unwrap() = sum;
            if sum == prev_deepness_level.len() as i32 {
                complete_count += 1;
            }
        }
        std::mem::swap(&mut cur_deepness_level, &mut prev_deepness_level);
        if complete_count == prev_deepness_level.len() {
            // all nodes are already connected with each other
            break;
        }
    }
    prev_deepness_level.into_iter().map(|branches| *branches.last().unwrap()).collect()
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![1; edges1.len() + 1];
        }
        let (connections1, connections2) = (edges2connections(edges1), edges2connections(edges2));
        let max_k = k as usize;
        let max_targets_from_tree2 = calculate_target_nodes(max_k - 1, &connections2)
            .into_iter()
            .max()
            .unwrap();
        let mut max_targets_in_tree1 = calculate_target_nodes(max_k, &connections1);
        for target in max_targets_in_tree1.iter_mut() {
            *target += max_targets_from_tree2;
        }
        max_targets_in_tree1
    }
}