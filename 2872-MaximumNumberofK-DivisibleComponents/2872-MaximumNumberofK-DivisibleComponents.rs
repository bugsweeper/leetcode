
struct Node {
    value: i32,
    visited_children: usize,
    neighbors: Vec<usize>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            visited_children: 0,
            neighbors: Vec::new(),
        }
    }
}

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut tree = values
            .into_iter()
            .map(|value| Node::new(value % k))
            .collect::<Vec<_>>();
        for edge in edges {
            if let [index1, index2] = &edge[..] {
                let index1 = *index1 as usize;
                let index2 = *index2 as usize;
                tree[index1].neighbors.push(index2);
                tree[index2].neighbors.push(index1);
            }
        }
        let mut stack = vec![(0usize)];
        let mut count = 1;
        while !stack.is_empty() {
            let node_index = stack.last().unwrap();
            let node = tree.get_mut(*node_index).unwrap();
            if node.neighbors.len() > node.visited_children {
                let next_child_index = node.neighbors[node.visited_children];
                node.visited_children += 1;
                if tree[next_child_index].visited_children == 0 {
                    stack.push(next_child_index);
                }
                continue;
            } else if stack.len() > 1 {
                if node.value % k == 0 {
                    count += 1;
                } else {
                    tree[stack[stack.len() - 2]].value += node.value % k;
                }
                stack.pop();
            } else {
                break;
            }
        }
        count
    }
}