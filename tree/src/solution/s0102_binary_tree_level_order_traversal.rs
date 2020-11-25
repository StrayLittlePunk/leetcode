// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

#![allow(unused)]
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;


impl Solution {
    // Time O(N) Space O(N)
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut res: Vec<Vec<i32>> = vec![];
        if root.is_none() {
          return res;
        }

        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root);
        while !deque.is_empty() {
          let mut current_level = vec![];
          let level_size = deque.len();

          for i in 0..level_size {
            if let Some(Some(node)) = deque.pop_front() {
              current_level.push(node.borrow().val);
              if !node.borrow().left.is_none() {
                deque.push_back(node.borrow().left.clone());
              }
              if !node.borrow().right.is_none() {
                deque.push_back(node.borrow().right.clone());
              }
            }
          }
          res.push(current_level);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
        assert_eq!(
            Solution::level_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
        assert_eq!(
            Solution::level_order(to_tree(vec![
                Some(1),
                Some(4),
                Some(2),
                Some(3),
                Some(5),
                None,
                Some(6)
            ])),
            vec![vec![1], vec![4, 2], vec![3, 5, 6]]
        );
    }
}
