use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        let mut direct_connections = vec![HashSet::new(); n];
        for edge in edges {
            let edge1 = edge[0] as usize;
            let edge2 = edge[1] as usize;
            direct_connections[edge1].insert(edge2);
            direct_connections[edge2].insert(edge1);
        }
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back((0usize/* jump index */, bob as usize/* position */));
        visited[bob as usize] = true;
        let mut bob_visit_history = HashMap::with_capacity(n);
        'bob_traverse: while let Some((mut jump_index, mut cell_index)) = queue.pop_front() {
            for &next_cell_index in &direct_connections[cell_index] {
                let next_cell_visited = unsafe{visited.get_unchecked_mut(next_cell_index)};
                if *next_cell_visited {
                    continue;
                }
                *next_cell_visited = true;
                if next_cell_index == 0 {
                    let mut single_chain = HashMap::new();
                    single_chain.insert(cell_index, jump_index);
                    while let Some(&prev_cell_index) = bob_visit_history.get(&cell_index) {
                        jump_index -= 1;
                        cell_index = prev_cell_index;
                        single_chain.insert(cell_index, jump_index);
                    }
                    std::mem::swap(&mut bob_visit_history, &mut single_chain);
                    break 'bob_traverse;
                }
                bob_visit_history.insert(next_cell_index, cell_index);
                queue.push_back((jump_index + 1, next_cell_index));
            }
        }
        // Now lets traverse Alice path
        visited = vec![false; n];
        let mut queue = VecDeque::with_capacity(n);
        queue.push_back((amount[0]/* net income */, 0usize/* jump index */, 0usize/* position */));
        visited[0] = true;
        let mut max_net_income = i32::MIN;
        while let Some((net_income, mut jump_index, cell_index)) = queue.pop_front() {
            let mut new_jumps = 0;
            jump_index += 1;
            for &next_cell_index in &direct_connections[cell_index] {
                let next_cell_visited = unsafe{visited.get_unchecked_mut(next_cell_index)};
                if *next_cell_visited {
                    continue;
                }
                *next_cell_visited = true;
                new_jumps += 1;
                let next_net_income = amount[next_cell_index];
                let net_income = net_income + match jump_index.cmp(bob_visit_history.get(&next_cell_index).unwrap_or(&n)) {
                    Ordering::Equal => next_net_income / 2,
                    Ordering::Greater => 0,
                    Ordering::Less => next_net_income,
                };
                queue.push_back((net_income, jump_index, next_cell_index));
            }
            if new_jumps == 0 {
                max_net_income = max_net_income.max(net_income);
            }
        }
        max_net_income
    }
}