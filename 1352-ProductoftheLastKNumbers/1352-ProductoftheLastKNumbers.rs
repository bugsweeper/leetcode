use std::cmp::Ordering;

#[derive(Default)]
struct ProductOfNumbers {
    non_zero_sequence_start: usize,
    products: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self::default()
    }
    
    fn add(&mut self, num: i32) {
        if self.products.is_empty() {
            self.products.push(num);
        } else {
            let previous_product = *self.products.last().unwrap();
            let product = if previous_product > 0 {previous_product * num} else {num};
            self.products.push(product);
        }
        if num == 0 {
            self.non_zero_sequence_start = self.products.len();
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        match k.cmp(&(self.products.len() - self.non_zero_sequence_start)) {
            Ordering::Equal => *self.products.last().unwrap(),
            Ordering::Greater => 0,
            Ordering::Less => *self.products.last().unwrap() / self.products[self.products.len() - k - 1],
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */