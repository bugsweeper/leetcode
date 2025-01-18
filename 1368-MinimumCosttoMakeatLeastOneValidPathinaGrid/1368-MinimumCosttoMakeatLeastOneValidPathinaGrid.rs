#[derive(PartialEq, Eq)]
struct Position {
    i: usize,
    j: usize,
    cost: i32,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then((self.i+self.j).cmp(&(other.i+other.j)))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const VISITED: i32 = 0;
const GO_RIGHT: i32 = 1;
const GO_LEFT: i32 = 2;
const GO_DOWN: i32 = 3;
const GO_UP: i32 = 4;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let mut two_side_queue = std::collections::VecDeque::new();
        two_side_queue.push_front(Position{i: 0, j: 0, cost: 0});
        let n = unsafe{grid.get_unchecked_mut(0)}.len();
        let mut costs = vec![vec![i32::MAX;n];m];
        loop {
            let Position{i, j, cost} = two_side_queue.pop_front().unwrap();
            if i == m - 1 && j == n - 1 {
                return cost;
            }
            let free_move = *unsafe{grid.get_unchecked(i).get_unchecked(j)};
            if free_move == VISITED {
                // Already visited => skipping
                continue;
            }
            // up?
            if i > 0 {
                let previous_expected_cost = unsafe{costs.get_unchecked_mut(i - 1).get_unchecked_mut(j)};
                if free_move == GO_UP {
                    if *previous_expected_cost > cost {
                        two_side_queue.push_front(Position{i: i - 1, j, cost});
                        *previous_expected_cost = cost;
                    }
                } else {
                    let expected_cost = cost + 1;
                    if *previous_expected_cost > expected_cost {
                        two_side_queue.push_back(Position{i: i - 1, j, cost: expected_cost});
                        *previous_expected_cost = expected_cost;
                    }
                }
            }
            // down?
            if i < m - 1 {
                let previous_expected_cost = unsafe{costs.get_unchecked_mut(i + 1).get_unchecked_mut(j)};
                if free_move == GO_DOWN {
                    if *previous_expected_cost > cost {
                        two_side_queue.push_front(Position{i: i + 1, j, cost});
                        *previous_expected_cost = cost;
                    }
                } else {
                    let expected_cost = cost + 1;
                    if *previous_expected_cost > expected_cost {
                        two_side_queue.push_back(Position{i: i + 1, j, cost: expected_cost});
                        *previous_expected_cost = expected_cost;
                    }
                }
            }
            let row = unsafe{costs.get_unchecked_mut(i)};
            // left?
            if j > 0 {
                let previous_expected_cost = unsafe{row.get_unchecked_mut(j - 1)};
                if free_move == GO_LEFT {
                    if *previous_expected_cost > cost {
                        two_side_queue.push_front(Position{i, j: j - 1, cost});
                        *previous_expected_cost = cost;
                    }
                } else {
                    let expected_cost = cost + 1;
                    if *previous_expected_cost > expected_cost {
                        two_side_queue.push_back(Position{i, j: j - 1, cost: expected_cost});
                        *previous_expected_cost = expected_cost;
                    }
                }
            }
            // right?
            if j < n - 1 {
                let previous_expected_cost = unsafe{row.get_unchecked_mut(j + 1)};
                if free_move == GO_RIGHT {
                    if *previous_expected_cost > cost {
                        two_side_queue.push_front(Position{i, j: j + 1, cost});
                        *previous_expected_cost = cost;
                    }
                } else {
                    let expected_cost = cost + 1;
                    if *previous_expected_cost > expected_cost {
                        two_side_queue.push_back(Position{i, j: j + 1, cost: expected_cost});
                        *previous_expected_cost = expected_cost;
                    }
                }
            }
            // mark as visited
            *unsafe{grid.get_unchecked_mut(i).get_unchecked_mut(j)} = VISITED;
        }
    }
}