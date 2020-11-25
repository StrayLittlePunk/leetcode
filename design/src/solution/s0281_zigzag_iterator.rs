#![allow(unused)]

use std::collections::VecDeque;
struct ZigzagIterator {
    list: Vec<Vec<i32>>,
    queue: VecDeque<(usize, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ZigzagIterator {
    /** initialize your data structure here. */

    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut list = vec![];
        list.push(v1);
        list.push(v2);
        let mut queue = VecDeque::new();
        for i in 0..list.len() {
            if list[i].len() > 0 {
                queue.push_back((i, 0));
            }
        }
        Self { list, queue }
    }

    fn next(&mut self) -> i32 {
        let (vec_p, elem_p) = self.queue.pop_front().unwrap();
        let next_elem_p = elem_p + 1;

        if next_elem_p < self.list[vec_p].len() {
            self.queue.push_back((vec_p, next_elem_p));
        }

        self.list[vec_p][elem_p]
    }

    fn has_next(&self) -> bool {
        self.queue.len() > 0
    }
}

/**
 * Your ZigzagIterator object will be instantiated and called as such:
 * let obj = ZigzagIterator::new(v1, v2);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_281() {}
}
