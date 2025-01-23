const EMPTY: i32 = 0;
const UNIQUE: i32 = 1;
const NON_UNIQUE: i32 = 2;
const UNIQUE_NOT_FOUND: usize = usize::MAX;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let mut grid = grid;
        let mut first_index = UNIQUE_NOT_FOUND;
        for (index, server) in unsafe{grid.get_unchecked_mut(0)}.iter_mut().enumerate() {
            if *server == 1 {
                sum += 1;
                match sum {
                    1 => first_index = index,
                    _ => *server = NON_UNIQUE,
                }
            }
        }
        let mut seen = std::mem::take(unsafe{grid.get_unchecked_mut(0)});
        if sum > 1 {
            *unsafe{seen.get_unchecked_mut(first_index)} = NON_UNIQUE;
        }
        for row in grid.into_iter().skip(1) {
            let mut row_sum = 0;
            first_index = UNIQUE_NOT_FOUND;
            for (seen, (index, server)) in seen.iter_mut().zip(row.into_iter().enumerate()) {
                if server == 0 {
                    continue;
                }
                row_sum += 1;
                match (*seen, row_sum) {
                    (EMPTY, 1) => {
                        first_index = index;
                        *seen = UNIQUE;
                    },
                    _ => {
                        *seen = NON_UNIQUE;
                    }
                }
            }
            if row_sum > 1 && first_index != UNIQUE_NOT_FOUND {
                *unsafe{seen.get_unchecked_mut(first_index)} = NON_UNIQUE;
            }
            sum += row_sum;
        }
        sum - seen.into_iter().filter(|&state| state == UNIQUE).count() as i32
    }
}