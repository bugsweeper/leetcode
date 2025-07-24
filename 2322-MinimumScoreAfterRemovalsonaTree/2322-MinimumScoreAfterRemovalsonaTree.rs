// Last updated: 25.07.2025, 02:04:01
use std::collections::{BTreeMap, VecDeque};

const CELL_SIZE: usize = u128::BITS as usize;
const CELL_BITS: usize = CELL_SIZE.ilog2() as usize;
const MAX_SCORE: i32 = 100_000_000;

#[inline]
fn mark(seen: &mut [u128], index: usize) {
    seen[index >> CELL_BITS] |= 1 << (index % CELL_SIZE);
}

// The main idea for this approach is to calculate all subtrees sum only once.
// Going from leafs to most far middle (central) node. Leafs are the nodes which connected to only one node.
impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        if nums.len() == 3 {
            return nums.iter().max().unwrap() - nums.iter().min().unwrap();
        }
        let mut connections = vec![vec![]; nums.len()];
        for edge in edges {
            let (node1, node2) = (edge[0] as usize, edge[1] as usize);
            // push neighbor and None as unset XOR sum
            connections[node1].push((node2, None));
            connections[node2].push((node1, None));
        }
        let mut remaining_neighbors = connections
            .iter()
            .map(|neighbors| neighbors.len())
            .collect::<Vec<_>>();
        for connection in &mut connections {
            connection.sort_unstable_by_key(|(node, _)| *node);
        }
        let total_sum = nums.iter().copied().reduce(|acc, e| acc ^ e).unwrap();
        let mut queue = VecDeque::with_capacity(nums.len());
        let mut sub_tree_sum2node_index: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        let mut subtree_nodes = vec![vec![0; nums.len().div_ceil(u128::BITS as usize)]; nums.len()];
        // processing only leafs
        let mut min_score = MAX_SCORE;
        for i in 0..nums.len() {
            let neighbors = connections[i].len();
            if neighbors > 1 {
                continue;
            }
            let leaf_num = nums[i];
            let parent_index = connections[i][0].0;
            let parent_connections = &mut connections[parent_index];
            let child_index = parent_connections
                .binary_search_by_key(&i, |(child_index, _)| *child_index)
                .unwrap();
            parent_connections[child_index].1 = Some(leaf_num);
            mark(&mut subtree_nodes[i], i);
            mark(&mut subtree_nodes[parent_index], i);
            for (&second_leaf_num, _) in
                sub_tree_sum2node_index.range(leaf_num - min_score + 1..leaf_num + min_score)
            {
                // for loops on leafs only
                let rest_tree: i32 = total_sum ^ leaf_num ^ second_leaf_num;
                min_score = min_score.min(
                    rest_tree.max(leaf_num).max(second_leaf_num)
                        - rest_tree.min(leaf_num).min(second_leaf_num),
                );
            }
            sub_tree_sum2node_index
                .entry(leaf_num)
                .and_modify(|indexes| indexes.push(i))
                .or_insert(vec![i]);
            let parent_neighbors = &mut remaining_neighbors[parent_index];
            *parent_neighbors -= 1;
            if *parent_neighbors == 1 {
                queue.push_back(parent_index);
            }
        }
        while let Some(index) = queue.pop_front() {
            let node_num = nums[index];
            let mut parent_index = None;
            let node_connections = &connections[index];
            let mut subtree_sum = node_num;
            for &(index, maybe_subtree_sum) in node_connections {
                if let Some(child_subtree_sum) = maybe_subtree_sum {
                    //subtree_sums.push(subtree_sum);
                    subtree_sum ^= child_subtree_sum;
                } else {
                    parent_index = Some(index);
                }
            }
            if parent_index.is_none() {
                // This is last node, can't deduce its edge to remove
                break;
            }
            let parent_index = parent_index.unwrap();
            // prepare parent for checks
            let parent_connections = &mut connections[parent_index];
            let node_index_in_parent_neighbors = parent_connections
                .binary_search_by_key(&index, |(child_index, _)| *child_index)
                .unwrap();
            parent_connections[node_index_in_parent_neighbors].1 = Some(subtree_sum);
            mark(&mut subtree_nodes[index], index);
            let (source, destination) = if index > parent_index {
                let (destination, source) = subtree_nodes.split_at_mut(index);
                (&source[0], &mut destination[parent_index])
            } else {
                let (source, destination) = subtree_nodes.split_at_mut(parent_index);
                (&source[index], &mut destination[0])
            };
            for (source, destination) in source.iter().zip(destination.iter_mut()) {
                *destination |= *source;
            }
            let parent_neighbors = &mut remaining_neighbors[parent_index];
            *parent_neighbors -= 1;
            if *parent_neighbors == 1 {
                queue.push_back(parent_index);
            }
            let opposite_subtree_sum = total_sum ^ subtree_sum;
            for (&sum, indexes2) in sub_tree_sum2node_index.range(
                subtree_sum.min(opposite_subtree_sum) - min_score + 1
                    ..subtree_sum.max(opposite_subtree_sum) + min_score,
            ) {
                for &index2 in indexes2 {
                    // check if previous branch is subtree of current branch
                    if subtree_nodes[index2]
                        .iter()
                        .zip(subtree_nodes[index].iter())
                        .any(|(&prev, &cur)| prev & cur != 0)
                    {
                        // if previous branch is subtree of current, then its removed edge splits current branch into two parts
                        let between = subtree_sum ^ sum;
                        let rest_tree = total_sum ^ subtree_sum;
                        min_score = min_score
                            .min(rest_tree.max(between).max(sum) - rest_tree.min(between).min(sum));
                    } else {
                        let rest_tree = total_sum ^ subtree_sum ^ sum;
                        min_score = min_score.min(
                            rest_tree.max(subtree_sum).max(sum)
                                - rest_tree.min(subtree_sum).min(sum),
                        );
                    }
                }
                if min_score == 0 {
                    return 0;
                }
            }
            sub_tree_sum2node_index
                .entry(subtree_sum)
                .and_modify(|indexes| indexes.push(index))
                .or_insert(vec![index]);
        }
        min_score
    }
}