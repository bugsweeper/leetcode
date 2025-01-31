use std::collections::VecDeque;

const WATER: i32 = 0;
const UNVISITED_ISLAND: i32 = 1;
const VISITED_INDEX_SHIFT: i32 = 2;

#[inline]
fn process_row(
    queue: &mut VecDeque<(usize, usize)>,
    row: &mut [i32],
    i: usize,
    j: usize,
    island_index: usize,
) -> i32 {
    let field = unsafe { row.get_unchecked_mut(j) };
    if *field != UNVISITED_ISLAND {
        return 0;
    }
    *field = VISITED_INDEX_SHIFT + island_index as i32;
    queue.push_back((i, j));
    1
}

#[inline]
fn process_grid(
    queue: &mut VecDeque<(usize, usize)>,
    grid: &mut [Vec<i32>],
    i: usize,
    j: usize,
    island_index: usize,
) -> i32 {
    process_row(
        queue,
        unsafe { grid.get_unchecked_mut(i) },
        i,
        j,
        island_index,
    )
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut grid = grid;
        let mut island2size = Vec::new();
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                let row = unsafe { grid.get_unchecked_mut(i) };
                let cell = unsafe { row.get_unchecked_mut(j) };
                if *cell != UNVISITED_ISLAND {
                    continue;
                }
                let island_index = island2size.len();
                let mut island_size = process_row(&mut queue, row, i, j, island_index);
                while let Some((i, j)) = queue.pop_front() {
                    if i > 0 {
                        island_size += process_grid(&mut queue, &mut grid, i - 1, j, island_index);
                    }
                    if i < n - 1 {
                        island_size += process_grid(&mut queue, &mut grid, i + 1, j, island_index);
                    }
                    let row = unsafe { grid.get_unchecked_mut(i) };
                    if j > 0 {
                        island_size += process_row(&mut queue, row, i, j - 1, island_index);
                    }
                    if j < n - 1 {
                        island_size += process_row(&mut queue, row, i, j + 1, island_index);
                    }
                }
                island2size.push(island_size);
            }
        }
        if island2size.is_empty() {
            return 1;
        }
        let mut max_size = *unsafe { island2size.get_unchecked(0) };
        for i in 0..n {
            for j in 0..n {
                if *unsafe { grid.get_unchecked_mut(i).get_unchecked_mut(j) } != WATER {
                    continue;
                }
                let (top_island, top_length) = if i > 0 {
                    let top_cell = *unsafe { grid.get_unchecked_mut(i - 1).get_unchecked_mut(j) };
                    if top_cell >= VISITED_INDEX_SHIFT {
                        (top_cell, *unsafe {
                            island2size.get_unchecked((top_cell - VISITED_INDEX_SHIFT) as usize)
                        })
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                };
                let (bottom_island, bottom_length) = if i < n - 1 {
                    let bottom_cell =
                        *unsafe { grid.get_unchecked_mut(i + 1).get_unchecked_mut(j) };
                    if bottom_cell >= VISITED_INDEX_SHIFT && bottom_cell != top_island {
                        (bottom_cell, *unsafe {
                            island2size.get_unchecked((bottom_cell - VISITED_INDEX_SHIFT) as usize)
                        })
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                };
                let row = unsafe { grid.get_unchecked_mut(i) };
                let (left_island, left_length) = if j > 0 {
                    let left_cell = *unsafe { row.get_unchecked_mut(j - 1) };
                    if left_cell >= VISITED_INDEX_SHIFT
                        && left_cell != top_island
                        && left_cell != bottom_island
                    {
                        (left_cell, *unsafe {
                            island2size.get_unchecked((left_cell - VISITED_INDEX_SHIFT) as usize)
                        })
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                };
                let right_length = if j < n - 1 {
                    let right_cell = *unsafe { row.get_unchecked_mut(j + 1) };
                    if right_cell >= VISITED_INDEX_SHIFT
                        && right_cell != top_island
                        && right_cell != bottom_island
                        && right_cell != left_island
                    {
                        *unsafe {
                            island2size.get_unchecked((right_cell - VISITED_INDEX_SHIFT) as usize)
                        }
                    } else {
                        0
                    }
                } else {
                    0
                };
                max_size =
                    max_size.max(1 + top_length + bottom_length + left_length + right_length);
            }
        }
        max_size
    }
}