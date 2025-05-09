// Last updated: 09.05.2025, 22:43:04
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(asteroids.len());
        'asteroids: for mut asteroid in asteroids {
            if asteroid < 0 {
                while let Some(prev_asteroid) = result.pop() {
                    if prev_asteroid < 0 {
                        result.push(prev_asteroid);
                        break;
                    }
                    if prev_asteroid > -asteroid {
                        asteroid = prev_asteroid;
                        break;
                    }
                    if prev_asteroid == -asteroid {
                        continue 'asteroids;
                    }
                }
            }
            result.push(asteroid);
        }
        result
    }
}