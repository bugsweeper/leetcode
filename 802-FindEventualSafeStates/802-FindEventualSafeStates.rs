#[derive(Clone, Copy, PartialEq)]
enum State {
    Unchecked,
    Safe,
    Unsafe,
}

fn checker(node_index: usize, graph: &Vec<Vec<i32>>, states: &mut Vec<State>) -> State {
    let state = unsafe{states.get_unchecked_mut(node_index)};
    if *state != State::Unchecked {
        return *state;
    }
    *state = State::Unsafe;
    for neighbor in &*unsafe{graph.get_unchecked(node_index)} {
        if checker(*neighbor as usize, graph, states) == State::Unsafe {
            return State::Unsafe
        }
    }
    *unsafe{states.get_unchecked_mut(node_index)} = State::Safe;
    return State::Safe;
}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut states = vec![State::Unchecked; graph.len()];
        let mut result = Vec::with_capacity(graph.len());
        for i in 0..graph.len() {
            if checker(i, &graph, &mut states) == State::Safe {
                result.push(i as i32);
            }
        }
        result
    }
}