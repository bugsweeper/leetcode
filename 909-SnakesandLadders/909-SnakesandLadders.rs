fn get_board_cell(index: usize, board: &[Vec<i32>]) -> i32 {
    let size = board.len();
    let rev_row = index / size;
    let row = size - rev_row - 1;
    let col = if rev_row % 2 == 0 {
        index % size
    } else {
        size - index % size - 1
    };
    board[row][col]
}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let size = board.len();
        let last_cell = size * size - 1;
        let mut step_queue = std::collections::VecDeque::with_capacity(last_cell + 1);
        step_queue.push_back((0, 0));
        let mut seen = vec![false; last_cell + 1];
        seen[0] = true;
        while let Some((index, mut spent_steps)) = step_queue.pop_front() {
            spent_steps += 1;
            for steps_forward in 1..=6 {
                let mut next_index = index + steps_forward;
                match get_board_cell(next_index, &board) {
                    -1 => {},
                    move_to_index => next_index = move_to_index as usize - 1,
                }
                if next_index == last_cell {
                    return spent_steps;
                }
                if !seen[next_index] {
                    step_queue.push_back((next_index, spent_steps));
                    seen[next_index] = true;
                }
            }
        }
        -1
    }
}