// Last updated: 16.06.2025, 13:37:06
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let (mut circular, mut square) =
            students
                .into_iter()
                .fold((0, 0), |(circular, square), student| {
                    if student == 0 {
                        (circular + 1, square)
                    } else {
                        (circular, square + 1)
                    }
                });
        for sandwich in sandwiches {
            if sandwich == 0 {
                if circular == 0 {
                    break;
                }
                circular -= 1;
            } else {
                if square == 0 {
                    break;
                }
                square -= 1;
            }
        }
        circular + square
    }
}