// Last updated: 21.09.2025, 17:07:03
use std::collections::{BTreeSet, HashMap};

struct MovieRentingSystem {
    prices: HashMap<(i32, i32), i32>,
    unrented: HashMap<i32, BTreeSet<(i32, i32)>>,
    rented: BTreeSet<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut prices = HashMap::with_capacity(entries.len());
        let mut unrented: HashMap<i32, BTreeSet<(i32, i32)>> =
            HashMap::with_capacity(entries.len());
        for entry in entries {
            let [shop, movie, price] = entry[..] else {
                unimplemented!();
            };
            prices.insert((shop, movie), price);
            unrented.entry(movie).or_default().insert((price, shop));
        }
        Self {
            prices,
            unrented,
            rented: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        if let Some(shops) = self.unrented.get(&movie) {
            shops.iter().take(5).map(|(_, shop)| *shop).collect()
        } else {
            Vec::new()
        }
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();
        self.unrented
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();
        self.rented.remove(&(price, shop, movie));
        self.unrented.get_mut(&movie).unwrap().insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop, movie])
            .collect()
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */