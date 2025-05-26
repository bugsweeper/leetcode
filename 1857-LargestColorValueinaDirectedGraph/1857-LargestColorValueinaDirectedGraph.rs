// Last updated: 26.05.2025, 16:10:09
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

#[derive(Clone, Copy)]
enum State {
    Pending,
    InProgress,
    Done,
}

fn recursive_path_value(colors: &[u8], children: &[Vec<usize>], node_values: &mut [Vec<i32>; ABC_LEN], state: &mut [State], node_index: usize) -> bool {
    match state[node_index] {
        State::Done => return true,
        State::InProgress => return false,
        _ => {}
    }
    state[node_index] = State::InProgress;
    for &child_node_index in &children[node_index] {
        if !recursive_path_value(colors, children, node_values, state, child_node_index) {
            return false;
        }
    }
    for color_values in node_values.iter_mut() {
        let mut max_value = 0;
        for &child_node_index in &children[node_index] {
            max_value = max_value.max(color_values[child_node_index]);
        }
        color_values[node_index] = max_value;
    }
    let current_node_color = (colors[node_index] - b'a') as usize;
    node_values[current_node_color][node_index] += 1;
    state[node_index] = State::Done;
    true
}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let mut children = vec![vec![]; colors.len()];
        for edge in edges {
            let [from, to] = edge[..] else {
                unimplemented!();
            };
            children[from as usize].push(to as usize);
        }
        let mut state = vec![State::Pending; colors.len()];
        // cache for the max value of chains starting at node i, where i is the index of node in colors
        // every cell contains max value for each color
        let mut node_values = [(); ABC_LEN].map(|_| vec![0; colors.len()]);
        for i in 0..colors.len() {
            if !recursive_path_value(colors.as_bytes(), &children, &mut node_values, &mut state, i) {
                return -1;
            }
        }
        node_values.into_iter().map(|color_values| color_values.into_iter().max().unwrap()).max().unwrap()
    }
}