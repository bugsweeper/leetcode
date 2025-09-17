// Last updated: 18.09.2025, 01:06:52
use std::cmp::Reverse;
use std::collections::{HashMap, BTreeSet};

struct FoodRatings {
    cuisine2index: HashMap<String, usize>,
    food2index: HashMap<String, (usize, i32)>,
    food_ratings: Vec<BTreeSet<(Reverse<i32>, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine2index = HashMap::with_capacity(cuisines.len());
        let mut food2index = HashMap::with_capacity(foods.len());
        let mut food_ratings = Vec::with_capacity(cuisines.len());
        for ((food, cuisine), rate) in foods.into_iter().zip(cuisines.into_iter()).zip(ratings) {
            let index = if let Some(&index) = cuisine2index.get(&cuisine) {
                index
            } else {
                let index = food_ratings.len();
                cuisine2index.insert(cuisine, index);
                food_ratings.push(BTreeSet::new());
                index
            };
            food2index.insert(food.clone(), (index, rate));
            food_ratings[index].insert((Reverse(rate), food));
        }
        Self {
            cuisine2index,
            food2index,
            food_ratings,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (index, old_rate) = self.food2index.get_mut(&food).unwrap();
        if *old_rate == new_rating {
            return;
        }
        let food_ratings = self.food_ratings.get_mut(*index).unwrap();
        let cell = (Reverse(*old_rate), food);
        food_ratings.remove(&cell);
        food_ratings.insert((Reverse(new_rating), cell.1));
        *old_rate = new_rating;
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let &index = self.cuisine2index.get(&cuisine).unwrap();
        self.food_ratings
            .get(index)
            .unwrap()
            .first()
            .unwrap()
            .1
            .clone()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */