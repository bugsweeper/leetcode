// Last updated: 02.06.2025, 16:55:32
use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut cities = HashSet::with_capacity(paths.len());
        for path in &paths {
            cities.insert(path[1].as_str());
        }
        for path in &paths {
            cities.remove(path[0].as_str());
        }
        for city in cities {
            return city.to_owned();
        }
        String::new()
    }
}