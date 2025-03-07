const NORTH: usize = 0;
const WEST: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;

const SHIFT: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let (mut x, mut y, mut direction) = (0, 0, NORTH);
        for instruction in instructions.as_bytes() {
            match instruction {
                b'G' => {
                    let shift = SHIFT[direction];
                    x += shift.0;
                    y += shift.1;
                }
                b'L' => direction = (direction + 1) % 4,
                b'R' => direction = (direction + 3) % 4,
                _ => unimplemented!(),
            }
        }
        direction != NORTH || (x == 0 && y == 0)
    }
}