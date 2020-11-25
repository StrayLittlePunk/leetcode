#![allow(unused)]

use std::collections::VecDeque;
struct MovingAverage {
    window: VecDeque<i32>,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        if size < 1 {
            return Self {
                window: VecDeque::new(),
                len: 0,
            };
        }
        Self {
            window: VecDeque::with_capacity(size as usize),
                len: size as usize,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.window.len() == self.len {
            self.window.pop_front();
            self.window.push_back(val);
            return self.window.iter().sum::<i32>() as f64 / self.window.len() as f64;
        }

        self.window.push_back(val);
        return self.window.iter().sum::<i32>() as f64 / self.window.len() as f64;
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {
    }
}
