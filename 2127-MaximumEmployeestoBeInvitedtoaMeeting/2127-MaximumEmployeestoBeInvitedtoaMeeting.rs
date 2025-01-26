use std::collections::{HashSet, VecDeque};

fn get_max_chain_to_node(dictionary: &[Vec<usize>], node: usize, exclude: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((node, 0));
    let mut max_length = 0;

    while let Some((node, length)) = queue.pop_front() {
        if node == exclude {
            continue;
        }
        max_length = max_length.max(length);
        for &neighbor in unsafe { dictionary.get_unchecked(node) } {
            queue.push_back((neighbor, length + 1));
        }
    }
    max_length
}

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut longest_cycle = 0;
        let mut visited = vec![false; n];
        let mut cycles_with_length_2 = Vec::new();
        // Searching longest cycle
        for i in 0..n {
            if *unsafe { visited.get_unchecked(i) } {
                continue;
            }
            let mut start = i;
            let mut current = i;
            let mut current_traversed_set = HashSet::new();
            while !*unsafe { visited.get_unchecked(current) } {
                *unsafe { visited.get_unchecked_mut(current) } = true;
                current_traversed_set.insert(current);
                current = *unsafe { favorite.get_unchecked(current) } as usize;
            }
            if current_traversed_set.contains(&current) {
                let mut current_traverse_length = current_traversed_set.len();
                while start != current {
                    current_traverse_length -= 1;
                    start = *unsafe { favorite.get_unchecked(start) } as usize;
                }
                longest_cycle = longest_cycle.max(current_traverse_length);
                if current_traverse_length == 2 {
                    cycles_with_length_2
                        .push((current, *unsafe { favorite.get_unchecked(current) }
                            as usize))
                }
            }
        }
        // Find sum of two longest non closed circles
        let mut inverted_favorite = vec![Vec::new(); n];
        for (from, to) in favorite.into_iter().enumerate() {
            unsafe { inverted_favorite.get_unchecked_mut(to as usize) }.push(from);
        }
        let mut sum_of_chains = 0;
        for (left, right) in cycles_with_length_2 {
            sum_of_chains += get_max_chain_to_node(&inverted_favorite, left, right)
                + get_max_chain_to_node(&inverted_favorite, right, left)
                + 2; // left and right itself
        }
        longest_cycle.max(sum_of_chains) as i32
    }
}