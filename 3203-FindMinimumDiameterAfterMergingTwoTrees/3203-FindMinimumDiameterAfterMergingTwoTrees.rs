fn edges_to_nodes(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut nodes = vec![Vec::new(); edges.len() + 1];
    for edge in edges {
        nodes[edge[0] as usize].push(edge[1] as usize);
        nodes[edge[1] as usize].push(edge[0] as usize);
    }
    nodes
}

fn get_diameter_and_longest_path(
    nodes: &Vec<Vec<usize>>,
    node_index: usize,
    parent_index: usize,
) -> (usize, usize) {
    let mut diameter = 0;
    let mut longest_path = 0;
    let mut second_path = 0;
    for &neighbor_index in &nodes[node_index] {
        if neighbor_index == parent_index {
            continue;
        }
        let (neighbor_diameter, neighbor_longest_path) =
            get_diameter_and_longest_path(nodes, neighbor_index, node_index);
        diameter = diameter.max(neighbor_diameter);
        second_path = second_path.max(neighbor_longest_path);
        if second_path > longest_path {
            std::mem::swap(&mut second_path, &mut longest_path);
        }
    }
    diameter = diameter.max(longest_path + second_path);
    (diameter, longest_path + 1)
}

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let nodes1 = edges_to_nodes(edges1);
        let nodes2 = edges_to_nodes(edges2);
        let (diameter1, _) = get_diameter_and_longest_path(&nodes1, 0, usize::MAX);
        let (diameter2, _) = get_diameter_and_longest_path(&nodes2, 0, usize::MAX);
        diameter1.max(diameter2.max(1 + diameter1.div_ceil(2) + diameter2.div_ceil(2))) as i32
    }
}