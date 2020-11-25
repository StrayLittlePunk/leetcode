
#![allow(unused)]
use std::collections::VecDeque;
struct MyStack {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

// Time Push O(N), Pop O(1), Peek O(1), Empty O(1)
// Space O(N)
impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::<i32>::new(),
        }
    }
    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
      let mut i = self.queue.len() - 1;
      while i > 0 {
        let tmp = self.queue.pop_front();
        self.queue.push_back(tmp.unwrap());
        i -= 1;
      }
    }

    fn pop(&mut self) -> i32 {
      self.queue.pop_front().unwrap()
    }


    fn top(&self) -> i32 {
      *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

// Time Push O(1), Pop O(1)~ O(N), Peek O(1)~ O(N), Empty O(1)
// Space O(N)
/* impl MyStack {
 *     fn new() -> Self {
 *         Self {
 *             queue: VecDeque::<i32>::new(),
 *         }
 *     }
 *
 *
 *     fn push(&mut self, x: i32) {
 *         self.queue.push_back(x)
 *     }
 *
 *     fn pop(&mut self) -> i32 {
 *       let mut i = self.queue.len() - 1;
 *       while i > 0 {
 *         let tmp = self.queue.pop_front();
 *         self.queue.push_back(tmp.unwrap());
 *         i -= 1;
 *       }
 *       self.queue.pop_front().unwrap()
 *     }
 *
 *
 *     fn top(&mut self) -> i32 {
 *       let mut i = self.queue.len() - 1;
 *       while i > 0 {
 *         let tmp = self.queue.pop_front();
 *         self.queue.push_back(tmp.unwrap());
 *         i -= 1;
 *       }
 *       let res = *self.queue.front().unwrap();
 *       let tmp = self.queue.pop_front();
 *       self.queue.push_back(tmp.unwrap());
 *       res
 *     }
 *
 *     fn empty(&self) -> bool {
 *         self.queue.is_empty()
 *     }
 * } */

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_225() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.empty(), false);
    }
}
