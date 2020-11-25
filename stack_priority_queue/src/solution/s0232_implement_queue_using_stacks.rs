#![allow(unused)]
struct MyQueue {
    stack: Vec<i32>,
    reverse_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */


// Time Push O(1), Pop O(1)~ O(N), Peek O(1)~ O(N), Empty O(1)
// Space O(N)
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: Vec::<i32>::new(),
            reverse_stack: Vec::<i32>::new(),
        }
    }

    fn flush_stack(&mut self) {
        while let Some(v) = self.stack.pop() {
            self.reverse_stack.push(v);
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x)
    }

    fn pop(&mut self) -> i32 {
        match self.reverse_stack.pop() {
            Some(e) => { e }
            None => {
                self.flush_stack();
                self.reverse_stack.pop().unwrap()
            }
        }
    }

    fn peek(&mut self) -> i32 {
        *match self.reverse_stack.last() {
            Some(e) => { e }
            None => {
                self.flush_stack();
                self.reverse_stack.last().unwrap()
            }
        }
    }

    fn empty(&self) -> bool {
        self.reverse_stack.is_empty() && self.stack.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);
    }
}
