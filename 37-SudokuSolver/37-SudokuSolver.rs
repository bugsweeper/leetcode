// Last updated: 01.09.2025, 02:22:43
// Algorithm Approach: Backtracking with optimizations
//
// The core strategy uses brute force with backtracking to try placing digits 1-9 in empty cells.
// When a placement creates a conflict (duplicate in row/column/3x3 box), we backtrack and try
// the next digit. This pruning eliminates entire branches of invalid solutions early.
//
// Optimization opportunities:
// - Process cells with more constraints first (fewer valid digits available)
// - Board transformation: transpose or reorder rows/columns to concentrate filled cells
//   in earlier positions, improving pruning effectiveness
// - Maintain constraint propagation to reduce search space

use std::cmp::Ordering;
use std::collections::BinaryHeap;

const SUBBOX_SIDE: usize = 3;
const SUBBOX_COUNT: usize = 3;
const BOARD_SIDE: usize = SUBBOX_SIDE * SUBBOX_COUNT;
const BOARD_SIZE: usize = BOARD_SIDE * BOARD_SIDE;
const FREE: u8 = b'.' & 0xf;
const DIGIT_OVERFLOW: u8 = 10;
const ROW_INDEX: [usize; BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8,
];
const SQUARE_INDEX: [usize; BOARD_SIZE] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4,
    4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6,
    6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8,
];
const COL_INDEX: [usize; BOARD_SIZE] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4,
    5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8,
];

#[derive(PartialEq, Eq)]
enum Direction {
    IncrementInPlace,
    Forward,
    Backward,
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_with_position = attach_cell_position(board);
        let fixed_rate_of_rows = sort_by_filled_cells(&mut board_with_position);
        board_with_position = transpose_board(board_with_position);
        let fixed_rate_of_cols = sort_by_filled_cells(&mut board_with_position);
        if fixed_rate_of_rows > fixed_rate_of_cols {
            board_with_position = transpose_board(board_with_position);
        }
        let mut seen_row = [[false; 10]; BOARD_SIDE];
        let mut seen_column = [[false; 10]; BOARD_SIDE];
        let mut seen_square = [[false; 10]; BOARD_SIDE];
        let mut fixed = [true; BOARD_SIZE];
        let mut cell = [0; BOARD_SIZE];
        let mut initial_position = [0; BOARD_SIZE];
        for (index, ((digit, position), initial_position)) in board_with_position
            .into_iter()
            .flatten()
            .zip(&mut initial_position)
            .enumerate()
        {
            *initial_position = position;
            if digit == FREE {
                fixed[index] = false;
            } else {
                cell[index] = digit;
                seen_row[ROW_INDEX[index]][digit as usize] = true;
                seen_square[SQUARE_INDEX[index]][digit as usize] = true;
                seen_column[COL_INDEX[index]][digit as usize] = true;
            }
        }
        let mut cur = 0;
        let mut state = if fixed[cur] {
            Direction::Forward
        } else {
            Direction::IncrementInPlace
        };
        loop {
            if state == Direction::IncrementInPlace {
                cell[cur] += 1;
                if cell[cur] == DIGIT_OVERFLOW {
                    state = Direction::Backward;
                } else {
                    let digit = cell[cur] as usize;
                    let seen_in_row = &mut seen_row[ROW_INDEX[cur]][digit];
                    if *seen_in_row {
                        continue;
                    }
                    let seen_in_square = &mut seen_square[SQUARE_INDEX[cur]][digit];
                    if *seen_in_square {
                        continue;
                    }
                    let seen_in_column = &mut seen_column[COL_INDEX[cur]][digit];
                    if *seen_in_column {
                        continue;
                    }
                    *seen_in_row = true;
                    *seen_in_square = true;
                    *seen_in_column = true;
                    state = Direction::Forward;
                }
            }
            match state {
                Direction::Backward => {
                    if cur == 0 {
                        panic!("There is no solution!");
                    }
                    cur -= 1;
                    if !fixed[cur] {
                        let digit = cell[cur] as usize;
                        seen_row[ROW_INDEX[cur]][digit] = false;
                        seen_square[SQUARE_INDEX[cur]][digit] = false;
                        seen_column[COL_INDEX[cur]][digit] = false;
                        state = Direction::IncrementInPlace;
                    }
                }
                Direction::Forward => {
                    cur += 1;
                    if cur == BOARD_SIZE {
                        break;
                    }
                    if !fixed[cur] {
                        cell[cur] = 0;
                        state = Direction::IncrementInPlace;
                    }
                }
                _ => {}
            }
        }
        for (value, position) in cell.into_iter().zip(initial_position) {
            board[position / BOARD_SIDE][position % BOARD_SIDE] = (value + b'0') as char;
        }
    }
}

fn attach_cell_position(board: &[Vec<char>]) -> Vec<Vec<(u8, usize)>> {
    let mut result = Vec::with_capacity(BOARD_SIDE);
    for (row, source) in board.iter().enumerate() {
        result.push(
            source
                .iter()
                .enumerate()
                .map(|(col, &data)| (data as u8 & 0xf, row * BOARD_SIDE + col))
                .collect(),
        );
    }
    result
}

struct IgnoreData<T> {
    fixed_count: u8,
    data: T,
}

impl<T> Ord for IgnoreData<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fixed_count.cmp(&other.fixed_count)
    }
}

impl<T> PartialOrd for IgnoreData<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for IgnoreData<T> {}

impl<T> PartialEq for IgnoreData<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fixed_count == other.fixed_count
    }
}

fn sort_by_filled_cells(board: &mut Vec<Vec<(u8, usize)>>) -> Vec<u8> {
    let mut heap = BinaryHeap::with_capacity(SUBBOX_COUNT);
    for _ in 0..SUBBOX_COUNT {
        let mut sub_heap = BinaryHeap::with_capacity(SUBBOX_SIDE);
        let mut total_fixed = 0;
        for _ in 0..SUBBOX_SIDE {
            let data = board.pop().unwrap();
            let fixed_count = data.iter().filter(|(digit, _)| *digit != FREE).count() as u8;
            total_fixed += fixed_count;
            sub_heap.push(IgnoreData { fixed_count, data });
        }
        heap.push(IgnoreData {
            fixed_count: total_fixed,
            data: sub_heap,
        });
    }
    let mut sorted_fixed_count = Vec::with_capacity(BOARD_SIDE);
    for _ in 0..SUBBOX_COUNT {
        let IgnoreData { mut data, .. } = heap.pop().unwrap();
        for _ in 0..SUBBOX_SIDE {
            let IgnoreData { fixed_count, data } = data.pop().unwrap();
            board.push(data);
            sorted_fixed_count.push(fixed_count);
        }
    }
    sorted_fixed_count
}

fn transpose_board(board: Vec<Vec<(u8, usize)>>) -> Vec<Vec<(u8, usize)>> {
    let mut transposed_board = (0..BOARD_SIDE)
        .map(|_| Vec::with_capacity(BOARD_SIDE))
        .collect::<Vec<_>>();
    for row in board {
        for (destination, cell) in transposed_board.iter_mut().zip(row) {
            destination.push(cell);
        }
    }
    transposed_board
}