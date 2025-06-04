// Last updated: 04.06.2025, 14:05:18
#[derive(Copy, Clone, PartialEq)]
enum State {
    Unknown,
    Special,
    NotSpecial,
}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        let mut state = vec![State::Unknown; n];
        let mut maybe_special_count = n;
        for row in mat {
            let mut ones_count = 0;
            let mut special_position = usize::MAX;
            for (index, (state, cell)) in state.iter_mut().zip(row).enumerate() {
                if cell == 0 {
                    continue;
                }
                match *state {
                    State::Unknown => {
                        if ones_count == 0 {
                            special_position = index;
                            *state = State::Special;
                        } else {
                            *state = State::NotSpecial;
                            maybe_special_count -= 1;
                            if maybe_special_count == 0 {
                                return 0;
                            }
                        }
                    }
                    State::Special => {
                        *state = State::NotSpecial;
                        maybe_special_count -= 1;
                        if maybe_special_count == 0 {
                            return 0;
                        }
                    }
                    _ => {}
                }
                ones_count += 1;
            }
            if special_position != usize::MAX && ones_count > 1 {
                state[special_position] = State::NotSpecial;
            }
        }
        state
            .into_iter()
            .filter(|&state| state == State::Special)
            .count() as i32
    }
}