#![allow(unused)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<(i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        // We always put the number onto the main stack.
        self.stack.push(x);
        // If the min stack is empty, or this number is smaller than
        // the top of the min stack, put it on with a count of 1.
        if self.min_stack.is_empty() || x < self.min_stack.last().unwrap().0 {
            self.min_stack.push((x, 1));
        }
        // Else if this number is equal to what's currently at the top
        // of the min stack, then increment the count at the top by 1.
        else if x == self.min_stack.last().unwrap().0 {
            let count = self.min_stack.last().unwrap();
            *(self.min_stack.last_mut().unwrap()) = (count.0, count.1 + 1);
        }
    }

    fn pop(&mut self) {
        // If the top of min stack is the same as the top of stack
        // then we need to decrement the count at the top by 1.
        if *self.stack.last().unwrap() == self.min_stack.last().unwrap().0 {
            let count = self.min_stack.last().unwrap();
            *(self.min_stack.last_mut().unwrap()) = (count.0, count.1 - 1);
        }

        // If the count at the top of min stack is now 0, then remove
        // that value as we're done with it.
        if self.min_stack.last().unwrap().1 == 0 {
            self.min_stack.pop();
        }
        // And like before, pop the top of the main stack.
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_stack[self.min_stack.len() - 1].0
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
