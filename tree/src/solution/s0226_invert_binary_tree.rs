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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      match root {
        None => None,
        Some(r) => {
          let left = Self::invert_tree(r.borrow().left.clone());
          let right = Self::invert_tree(r.borrow().right.clone());
          r.borrow_mut().left = right;
          r.borrow_mut().right = left;
          Some(r)
        }
      }
    }
    // Time O(N) Space O(N)
    pub fn invert_tree_i(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let r = root.clone();
        deque.push_back(r);
        while let Some(Some(cur)) = deque.pop_front() {
          let l = cur.borrow().left.clone();
          let r = cur.borrow().right.clone();
          cur.borrow_mut().left = r;
          cur.borrow_mut().right = l;
          if cur.borrow().left.is_some() {
            deque.push_back(cur.borrow().left.clone());
          }
          if cur.borrow().right.is_some() {
            deque.push_back(cur.borrow().right.clone());
          }
        }
        
      root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree_i(to_tree(vec![
                Some(4),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(6),
                Some(9)
            ])),
            to_tree(vec![
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1)
            ])
        );
    }
}
