// Last updated: 20.04.2025, 08:19:52
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let n = answers.len();
        let mut answer_count = vec![0; 1000];
        for answer in answers {
            answer_count[answer as usize] += 1;
        }
        let mut additional_count = 0;
        for (answer, answer_count) in answer_count.into_iter().enumerate().skip(1) {
            if answer_count == 0 {
                continue;
            }
            additional_count += (answer + 1) - (answer_count - 1) % (answer + 1) - 1;
        }
        (n + additional_count) as i32
    }
}