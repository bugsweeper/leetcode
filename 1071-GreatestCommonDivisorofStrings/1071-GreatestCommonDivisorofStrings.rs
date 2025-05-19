// Last updated: 19.05.2025, 15:19:08
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut candy = 1;
        let mut people = vec![0; num_people as usize];
        let mut candies = candies;
        loop {
            for human in people.iter_mut() {
                let give = candy.min(candies);
                *human += give;
                candies -= give;
                if candies == 0 {
                    return people;
                }
                candy += 1;
            }
        }
        people
    }
}