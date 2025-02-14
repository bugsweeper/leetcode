use std::collections::VecDeque;

struct MyQueue {
    stack: VecDeque<i32>,
    temp_stack: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            stack: VecDeque::with_capacity(100),
            temp_stack: VecDeque::with_capacity(100),
        }        
    }
    
    fn push(&mut self, x: i32) {
        while !self.stack.is_empty() {
            self.temp_stack.push_back(self.stack.pop_back().unwrap());
        }
        self.stack.push_back(x);
        while !self.temp_stack.is_empty() {
            let value = self.temp_stack.pop_back().unwrap();
            self.stack.push_back(value);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.stack.pop_back().unwrap()
    }
    
    fn peek(&self) -> i32 {
        *self.stack.back().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */